use std::{
    fs::File,
    io::{self, Write},
    mem,
};

use crate::delimiter::DelimiterTable;

#[derive(Debug, thiserror::Error)]
pub enum WriterError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
    #[error("{0} can't be converted to a digit")]
    ParseInt(char),
}

pub struct Writer {
    debug: bool,
    pretty: bool,
    buffer: u8,
    buffer_size_used: u8,
    buffer_size: u8,
    bits_buffer: Vec<u8>,
    bin_file: File,
    debug_file: Option<File>,
}

impl Writer {
    pub fn new(debug: bool, pretty: bool) -> Result<Self, WriterError> {
        Ok(Self {
            debug,
            pretty,
            buffer: 0,
            buffer_size_used: 0,
            buffer_size: 8,
            bits_buffer: Vec::new(),
            bin_file: File::create("output.bin")?,
            debug_file: if debug {
                Some(File::create("debug.txt")?)
            } else {
                None
            },
        })
    }

    pub fn flush(&mut self) -> Result<(), io::Error> {
        if self.buffer_size_used > 0 {
            let remaining_bits = self.buffer_size - self.buffer_size_used;
            if remaining_bits > 0 {
                self.buffer <<= remaining_bits;
            }
            self.bin_file.write_all(&[self.buffer])?;
            self.buffer = 0;
            self.buffer_size_used = 0;
        }
        Ok(())
    }

    pub fn add_to_buffer(&mut self, mut data: u32, mut bit_count: u8) -> Result<(), WriterError> {
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
            self.flush()?;
        }
        self.buffer = (self.buffer << bit_count) | data as u8;
        self.buffer_size_used += bit_count;
        Ok(())
    }

    pub fn write1(&mut self, data: u32) -> Result<(), WriterError> {
        self.add_to_buffer(data, 1)?;
        if self.debug {
            // convert u8 to binary string padded to bit_count in the front
            let binary_string = format!("{:0>width$b}", data, width = 1 as usize);
            let mut debug_file: &File;
            match &self.debug_file {
                Some(file) => debug_file = file,
                None => return Ok(()),
            }
            debug_file.write_all(binary_string.as_bytes())?;
        }
        Ok(())
    }

    pub fn write(&mut self, data: u32, bit_count: u8) -> Result<(), WriterError> {
        let binary = format!("{:0>width$b}", data, width = bit_count as usize);
        for bit in binary.chars() {
            self.bits_buffer
                .push(bit.to_digit(10).ok_or(WriterError::ParseInt(bit))? as u8);
        }
        Ok(())
    }

    pub fn patch(&mut self, location: u32, data: u32, bit_count: u8) -> Result<(), WriterError> {
        let binary = format!("{:0>width$b}", data, width = bit_count as usize);
        for (i, bit) in binary.chars().enumerate() {
            self.bits_buffer[location as usize + i] =
                bit.to_digit(10).ok_or(WriterError::ParseInt(bit))? as u8;
        }
        Ok(())
    }

    pub fn done(&mut self, delimiter_table: &mut DelimiterTable) -> Result<(), WriterError> {
        let bits_buffer = mem::take(&mut self.bits_buffer);
        let len = bits_buffer.len();
        bits_buffer.iter().enumerate().try_for_each(|(i, bit)| {
            self.write1(*bit as u32)?;
            if let None = delimiter_table.get_current() {
                delimiter_table.next();
            }
            if self.debug
                && self.pretty
                && let Some(delimiter) = delimiter_table.get_current()
            {
                if (delimiter.address as usize) == i + 1
                    && let Some(debug_file) = self.debug_file.as_mut()
                {
                    debug_file.write_all(delimiter.symbol.as_bytes())?;
                    delimiter_table.next();
                }
            }
            Ok::<(), WriterError>(())
        })?;
        self.close(len as u32)?;
        Ok(())
    }

    pub fn close(&mut self, bits_written: u32) -> Result<(), WriterError> {
        self.flush()?;
        self.add_to_buffer(bits_written, 32)?;
        self.flush()?;
        println!("Binary written to file: output.bin");
        if let Some(debug_file) = self.debug_file.as_mut() {
            debug_file.write_all(b"\n")?;
            debug_file.flush()?;
            println!("Debug file written to debug.txt");
        }
        Ok(())
    }
}
