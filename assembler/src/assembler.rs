use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Read},
};
use thiserror::Error;

use crate::{
    bin_generator::{BinGenError, BinGenerator},
    parser::{Parser, ParserError},
    semantic_analyzer::{SemanticAnalyzer, SemanticError},
    writer::{Writer, WriterError},
};

#[derive(Debug, Error)]
pub enum AssemblerError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Parser error: {0}")]
    Instruction(#[from] ParserError),
    #[error("Writer error: {0}")]
    WriterError(#[from] WriterError),
    #[error("Symbol {0} not found")]
    MissingSymbol(String),
    #[error("Error in semantic analysis: {0}")]
    Semantic(#[from] SemanticError),
    #[error("Binary generation error: {0}")]
    BinGen(#[from] BinGenError),
}

pub struct MyAssembler {
    // To store the symbols
    symtab: HashMap<String, u32>,
    location_counter: u32,
    writer: Writer,
    debug: bool,
    // To store the instruction whose symbols could not be resolved yet
    tii: HashMap<String, Vec<u32>>,
    parser: Parser,
    semantic_analyzer: SemanticAnalyzer,
    bin_generator: BinGenerator,
}

impl MyAssembler {
    pub fn new(debug: bool, pretty: bool) -> Result<Self, AssemblerError> {
        Ok(Self {
            location_counter: 0,
            symtab: HashMap::new(),
            writer: Writer::new(debug, pretty)?,
            debug,
            tii: HashMap::new(),
            parser: Parser::new(),
            semantic_analyzer: SemanticAnalyzer::new(debug),
            bin_generator: BinGenerator::new(),
        })
    }

    pub fn print_symtab(&self) {
        println!("Symbol Table:");
        println!("{:#?}", self.symtab);
    }

    pub fn assemble(&mut self, file_name: &str) -> Result<(), AssemblerError> {
        let file = File::open(file_name)?;
        let mut buffer = String::new();

        let mut reader = BufReader::new(file);
        println!("Assembly file: {}", file_name);
        reader.read_to_string(&mut buffer)?;
        for line in buffer.lines() {
            let instruction = self.parser.parse(line)?;

            let instruction = self.semantic_analyzer.analyze(
                instruction,
                line.to_string(),
                &mut self.symtab,
                &mut self.tii,
                &mut self.location_counter,
                &mut self.writer,
            )?;
            self.bin_generator
                .generate_binary(instruction, &mut self.writer, &mut self.location_counter)?;
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
