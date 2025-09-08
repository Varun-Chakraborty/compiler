use std::{fs::File, io::Write};
pub struct Writer {
    debug: bool,
    pretty: bool,
    buffer: u8,
    buffer_size_used: u8,
    buffer_size: u8,
    bits_written: u8,
    bin_file: File,
    debug_file: Option<File>,
}

impl Writer {
    pub fn new(debug: bool, pretty: bool) -> Self {
        return Self {
            debug,
            pretty,
            buffer: 0,
            buffer_size_used: 0,
            buffer_size: 8,
            bits_written: 0,
            bin_file: File::create("output.bin").unwrap(),
            debug_file: if debug {
                Some(File::create("debug.txt").unwrap())
            } else {
                None
            },
        };
    }

    pub fn flush(&mut self) {
        if self.buffer_size_used > 0 {
            let remaining_bits = self.buffer_size - self.buffer_size_used;
            if remaining_bits > 0 {
                self.buffer <<= remaining_bits;
            }
            self.bin_file.write_all(&[self.buffer]).unwrap();
            self.buffer = 0;
            self.buffer_size_used = 0;
        }
    }

    pub fn add_to_buffer(&mut self, mut data: u32, mut bit_count: u8) {
        while bit_count + self.buffer_size_used >= self.buffer_size {
            let remaining_bits = self.buffer_size - self.buffer_size_used;

            self.buffer = if self.buffer_size_used == 0 {
                0
            } else {
                self.buffer << remaining_bits
            } | (data >> (bit_count - remaining_bits)) as u8;

            self.buffer_size_used += remaining_bits;
            data &= (1 << bit_count - remaining_bits) - 1;
            bit_count -= remaining_bits;
            self.flush();
        }
        self.buffer = (self.buffer << bit_count) | data as u8;
        self.buffer_size_used += bit_count;
    }

    pub fn write(&mut self, data: u32, bit_count: u8) -> () {
        self.add_to_buffer(data, bit_count);
        self.bits_written += bit_count;
        if self.debug {
            // convert u8 to binary string padded to bit_count in the front
            let binary_string = format!("{:0>width$b}", data, width = bit_count as usize);
            let mut debug_file: &File;
            match &self.debug_file {
                Some(file) => debug_file = file,
                None => return,
            }
            debug_file.write_all(binary_string.as_bytes()).unwrap();
            if self.pretty {
                debug_file.write_all(b" ").unwrap();
            }
        }
    }

    pub fn new_line(&mut self) {
        if self.debug && self.pretty {
            if let Some(debug_file) = self.debug_file.as_mut() {
                debug_file.write_all(b"\n").unwrap();
            }
        }
    }

    pub fn close(&mut self) {
        self.flush();
        self.add_to_buffer(self.bits_written.into(), 8);
        self.flush();
        println!("Binary file written to output.bin");
        if let Some(debug_file) = self.debug_file.as_mut() {
            debug_file.write_all(b"\n").unwrap();
            debug_file.flush().unwrap();
            println!("Debug file written to debug.txt");
        }
    }
}
