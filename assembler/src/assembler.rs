use std::{collections::HashMap, fs::File, io::Read};
use thiserror::Error;

use crate::{
    instruction::{Instruction, InstructionError},
    writer::{Writer, WriterError},
};

#[derive(Debug, Error)]
pub enum AssemblerError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Instruction error: {0}")]
    Instruction(#[from] InstructionError),
    #[error("{0}")]
    WriterError(#[from] WriterError),
    #[error("Symbol {0} not found")]
    MissingSymbol(String),
}

pub struct MyAssembler {
    // To store the symbols
    symtab: HashMap<String, u32>,
    location_counter: u32,
    writer: Writer,
    debug: bool,
    // To store the instruction whose symbols could not be resolved yet
    tii: HashMap<String, Vec<u32>>,
}

impl MyAssembler {
    pub fn new(debug: bool, pretty: bool) -> Result<Self, AssemblerError> {
        Ok(Self {
            location_counter: 0,
            symtab: HashMap::new(),
            writer: Writer::new(debug, pretty)?,
            debug,
            tii: HashMap::new(),
        })
    }

    pub fn print_symtab(&self) {
        println!("Symbol Table:");
        println!("{:?}", self.symtab);
    }

    pub fn assemble(&mut self, file_name: &str) -> Result<(), AssemblerError> {
        use std::io::BufReader;
        let file = File::open(file_name)?;
        let mut buffer = String::new();
        let mut reader = BufReader::new(file);
        println!("Assembly file: {}", file_name);
        reader.read_to_string(&mut buffer)?;
        for line in buffer.lines() {
            let mut instruction = Instruction::new(
                &mut self.writer,
                &mut self.location_counter,
                &mut self.symtab,
                self.debug,
                &mut self.tii,
            );
            instruction.parse(line)?;
            instruction.done()?;
        }
        if self.debug {
            self.print_symtab();
        }
        if !self.tii.is_empty() {
            return Err(AssemblerError::MissingSymbol(
                self.tii.keys().next().unwrap().to_string(),
            ));
        }
        self.writer.done()?;
        println!("Assembly completed.");
        Ok(())
    }
}
