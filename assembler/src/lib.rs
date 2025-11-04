mod bin_generator;
mod delimiter;
mod instruction;
mod parser;
mod semantic_analyzer;
pub mod writer;

use args::Args;
use logger::{LogTo, Logger, LoggerError};
use std::{collections::HashMap, mem};
use thiserror::Error;

use crate::{
    bin_generator::{BinGenError, BinGenerator},
    delimiter::DelimiterTable,
    parser::{Parser, ParserError},
    semantic_analyzer::{SemanticAnalyzer, SemanticError},
};

#[derive(Debug, Error)]
pub enum AssemblerError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Parser error: {0}")]
    Instruction(#[from] ParserError),
    #[error("Symbol {0} not found")]
    MissingSymbol(String),
    #[error("Error in semantic analysis: {0}")]
    Semantic(#[from] SemanticError),
    #[error("Binary generation error: {0}")]
    BinGen(#[from] BinGenError),
    #[error("Logger error: {0}")]
    Logger(#[from] LoggerError),
    #[error("Unknown error: {msg}")]
    Unknown{ msg: String },
}

pub struct MyAssembler {

    symtab: HashMap<String, u32>,
    location_counter: u32,
    debug: bool,

    tii: HashMap<String, Vec<u32>>,
    parser: Parser,
    semantic_analyzer: SemanticAnalyzer,
    bin_generator: BinGenerator,
    delimiter_table: DelimiterTable,
    logger: Logger,
}

impl MyAssembler {
    pub fn new(args: &Args) -> Result<Self, AssemblerError> {
        Ok(Self {
            location_counter: 0,
            symtab: HashMap::new(),
            debug: args.debug,
            tii: HashMap::new(),
            parser: Parser::new(),
            semantic_analyzer: SemanticAnalyzer::new(args.debug),
            bin_generator: BinGenerator::new(),
            delimiter_table: DelimiterTable::new(),
            logger: Logger::new(
                if let Some(filename) = args.filename.clone() {
                    filename
                } else {
                    String::from("assembler.txt")
                },
                args.path.clone(),
                if let Some(log_to) = args.log_to.clone() {
                    if log_to == "file" {
                        LogTo::File
                    } else {
                        LogTo::Console
                    }
                } else {
                    LogTo::Console
                },
            )?,
        })
    }

    pub fn print_symtab(&self) {
        println!("Symbol Table:");
        println!("{:#?}", self.symtab);
    }

    pub fn assemble(&mut self, assembly_program: String) -> Result<(Vec<u8>, DelimiterTable), AssemblerError> {
        for line in assembly_program.lines() {
            let instruction = self.parser.parse(line)?;

            let instruction = match self.semantic_analyzer.analyze(
                instruction,
                line.to_string(),
                &mut self.symtab,
                &mut self.tii,
                &mut self.location_counter,
                &mut self.bin_generator,
                &mut self.logger,
            )? {
                Some(instruction) => instruction,
                None => continue,
            };

            self.bin_generator.generate_binary(
                instruction,
                &mut self.location_counter,
                &mut self.delimiter_table,
            )?;
        }
        if self.debug {
            self.print_symtab();
        }
        if !self.tii.is_empty() {
            return Err(AssemblerError::MissingSymbol(
                self.tii.keys().next().ok_or(AssemblerError::Unknown { msg: "Can't get next key".to_string() })?.to_string(),
            ));
        }
        println!("Assembly completed.");
        let delimiter_table = mem::take(&mut self.delimiter_table);
        Ok((self.bin_generator.get_binary(), delimiter_table))
    }
}
