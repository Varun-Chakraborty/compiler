use std::{collections::HashMap, error::Error, fs::File, io::Read};
use crate::{instruction::{Instruction}, writer::Writer};

pub struct MyAssembler {
    symtab: HashMap<String, u32>,
    location_counter: u32,
    writer: Writer,
    debug: bool
}

impl MyAssembler {
    pub fn new(debug: bool, pretty: bool) -> Result<Self, Box<dyn Error>> {
        return Ok(Self {
            location_counter: 0,
            symtab: HashMap::new(),
            writer: Writer::new(debug, pretty)?,
            debug
        });
    }

    pub fn print_symtab(&self) {
        println!("Symbol Table:");
        println!("{:?}", self.symtab);
    }

    pub fn assemble(&mut self, file_name: &str) -> Result<(), Box<dyn Error>>{
        use std::io::{BufReader};
        let file = File::open(file_name)?;
        let mut buffer = String::new();
        let mut reader = BufReader::new(file);
        let mut is_comment = false;
        println!("Assembly file: {}", file_name);
        match reader.read_to_string(&mut buffer) {
            Ok(_) => (),
            Err(e) => return Err(format!("Failed to read file: {}", e).into())
        }
        let mut instruction = Instruction::new(&mut self.writer, &mut self.location_counter, &mut self.symtab, self.debug);
        let mut token = String::new();
        for c in buffer.chars() {
            match c {
                ',' | ' ' | '\t' => {
                    if is_comment {
                        continue;
                    }
                    instruction.add_token(token)?;
                },
                ';' | '\n' => {
                    if !is_comment {
                        instruction.add_token(token)?;
                    }
                    if c == '\n' {
                        instruction.done()?;
                        is_comment = false;
                        instruction = Instruction::new(&mut self.writer, &mut self.location_counter, &mut self.symtab, self.debug);
                    }
                    if c == ';' {
                        is_comment = true;
                    }
                },
                _ => {
                    if !is_comment {
                        token.push(c);
                    }
                    continue;
                }
            }
            token = String::new();
        }
        instruction.add_token(token)?;
        instruction.done()?;
        if self.debug { 
            self.print_symtab();
        }
        self.writer.close()?;
        println!("Assembly completed.");
        Ok(())
    }
}