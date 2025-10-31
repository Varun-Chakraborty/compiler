use args::Args;
use logger::{LogTo, Logger, LoggerError};
use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Read},
};
use thiserror::Error;

use crate::{
    bin_generator::{BinGenError, BinGenerator},
    delimiter::DelimiterTable,
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
    #[error("Logger error: {0}")]
    Logger(#[from] LoggerError),
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
    delimiter_table: DelimiterTable,
    logger: Logger,
}

impl MyAssembler {
    pub fn new(args: &Args) -> Result<Self, AssemblerError> {
        Ok(Self {
            location_counter: 0,
            symtab: HashMap::new(),
            writer: Writer::new(args.debug, args.pretty)?,
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
                    "assembler.txt".to_string()
                },
                if let Some(path) = args.path.clone() {
                    path
                } else {
                    "./logs/".to_string()
                },
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

    pub fn assemble(&mut self, file_name: &str) -> Result<(), AssemblerError> {
        let file = File::open(file_name)?;
        let mut buffer = String::new();

        let mut reader = BufReader::new(file);
        println!("Assembly file: {}", file_name);
        reader.read_to_string(&mut buffer)?;
        for line in buffer.lines() {
            let instruction = self.parser.parse(line)?;

            let instruction = match self.semantic_analyzer.analyze(
                instruction,
                line.to_string(),
                &mut self.symtab,
                &mut self.tii,
                &mut self.location_counter,
                &mut self.writer,
                &mut self.logger,
            )? {
                Some(instruction) => instruction,
                None => continue,
            };

            self.bin_generator.generate_binary(
                instruction,
                &mut self.writer,
                &mut self.location_counter,
                &mut self.delimiter_table,
            )?;
        }
        if self.debug {
            self.print_symtab();
        }
        if !self.tii.is_empty() {
            return Err(AssemblerError::MissingSymbol(
                self.tii.keys().next().unwrap().to_string(),
            ));
        }
        self.writer.done(&mut self.delimiter_table)?;
        println!("Assembly completed.");
        Ok(())
    }
}
