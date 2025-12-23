mod encoder;
mod lexer;
mod parser;
pub mod writer;

use thiserror::Error;

use self::{
    encoder::{Encoder, EncoderError, delimiter::DelimiterTable},
    lexer::{Lexer, LexerError},
    parser::{Parser, ParserError},
};

#[derive(Debug, Error)]
pub enum AssemblerError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Unknown error: {msg}")]
    Unknown { msg: String },
    #[error("Lexer error: {0}")]
    Lexer(#[from] LexerError),
    #[error("Parser error: {0}")]
    Parser(#[from] ParserError),
    #[error("Encoder error: {0}")]
    Encoder(#[from] EncoderError),
}

pub struct MyAssembler {}

impl MyAssembler {
    pub fn new() -> Result<Self, AssemblerError> {
        Ok(Self {})
    }

    pub fn assemble(
        &mut self,
        assembly_program: &str,
    ) -> Result<(Vec<u8>, DelimiterTable), AssemblerError> {
        let mut lexer = Lexer::new();
        let mut parser = Parser::new();
        let mut encoder = Encoder::new();

        println!("Assembling...");
        let tokens = lexer.lex(assembly_program)?;
        let instructions = parser.parse(tokens)?;
        let (binary, delimiter_table) = encoder.encode(instructions)?;

        println!("{:?}", binary);

        Ok((binary, delimiter_table))
    }
}
