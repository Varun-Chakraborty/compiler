use std::{
    fs::File,
    io::{self, Write},
};

use crate::delimiter::DelimiterTable;

#[derive(Debug, thiserror::Error)]
pub enum WriterError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
}

pub struct Writer {
    debug: bool,
    pretty: bool,
    bin_file: File,
    debug_file: Option<File>,
}

impl Writer {
    pub fn new(debug: bool, pretty: bool) -> Result<Self, WriterError> {
        Ok(Self {
            debug,
            pretty,
            bin_file: File::create("output.bin")?,
            debug_file: if debug {
                Some(File::create("debug.txt")?)
            } else {
                None
            },
        })
    }

    pub fn write(
        &mut self,
        bytes_stream: Vec<u8>,
        delimiter_table: &mut DelimiterTable,
    ) -> Result<(), WriterError> {
        let mut bits_written = 0 as usize;
        if let None = delimiter_table.get_current() {
            delimiter_table.next();
        }
        bytes_stream.iter().try_for_each(|&byte| {
            self.bin_file.write_all(&[byte])?;
            if self.debug {
                let mut debug_file: &File;
                match &self.debug_file {
                    Some(file) => debug_file = file,
                    None => return Ok(()),
                }
                for bit in format!("{:0>8b}", byte).chars() {
                    if self.pretty {
                        if let Some(current) = delimiter_table.get_current() {
                            if bits_written == current.address {
                                debug_file.write_all(current.symbol.as_bytes())?;
                                delimiter_table.next();
                            }
                        }
                        bits_written += 1;
                    }
                    debug_file.write_all(bit.to_string().as_bytes())?;
                }
            }
            Ok::<(), WriterError>(())
        })?;
        Ok(())
    }
}
