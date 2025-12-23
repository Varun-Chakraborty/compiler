pub mod instruction;
mod semantic_parser;
mod syntactic_parser;

use crate::{
    lexer::token::TokenStream,
    parser::{
        instruction::Instruction,
        semantic_parser::{SemanticError, SemanticParser},
        syntactic_parser::{SyntacticError, SyntacticParser},
    },
};

#[derive(Debug, thiserror::Error)]
pub enum ParserError {
    #[error("Syntactic parsing error: {0}")]
    SyntacticParsing(#[from] SyntacticError),
    #[error("Semantic parsing error: {0}")]
    SemanticParsing(#[from] SemanticError),
}

pub struct Parser {
    syntactic_parser: SyntacticParser,
    semantic_parser: SemanticParser,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            syntactic_parser: SyntacticParser::new(),
            semantic_parser: SemanticParser::new(),
        }
    }

    pub fn parse(
        &mut self,
        tokens: TokenStream,
    ) -> Result<Vec<Instruction>, ParserError> {
        let statements = self.syntactic_parser.parse(tokens)?;
        let instructions = self.semantic_parser.parse(statements)?;
        Ok(instructions)
    }
}

#[cfg(test)]
mod tests {
    use super::{super::lexer::token::Token, Parser, TokenStream, instruction::InstructionField};

    #[test]
    fn test_parser() {
        let mut parser = Parser::new();
        let mut tokens = TokenStream::new();
        tokens.push(Token::Identifier("MOVE".to_string()));
        tokens.push(Token::Symbol(':'));
        tokens.push(Token::Identifier("MOVER".to_string()));
        tokens.push(Token::Identifier("R0".to_string()));
        tokens.push(Token::Symbol(','));
        tokens.push(Token::Identifier("0".to_string()));
        tokens.push(Token::Newline);
        tokens.push(Token::Identifier("MOVE1".to_string()));
        tokens.push(Token::Symbol(':'));
        tokens.push(Token::Identifier("MOVER".to_string()));
        tokens.push(Token::Identifier("R0".to_string()));
        tokens.push(Token::Symbol(','));
        tokens.push(Token::Identifier("0".to_string()));
        tokens.push(Token::Newline);
        tokens.push(Token::Eof);
        let instructions = parser.parse(tokens).unwrap();
        assert_eq!(instructions.len(), 2);
        let instruction = &instructions[0];
        assert_eq!(
            instruction.opcode,
            InstructionField {
                value: 1,
                bit_count: 4
            }
        );
        let operands = instruction.operands.as_ref().unwrap();
        assert_eq!(operands.len(), 2);
        assert_eq!(
            operands[0],
            InstructionField {
                value: 0,
                bit_count: 2
            }
        );
        assert_eq!(
            operands[1],
            InstructionField {
                value: 0,
                bit_count: 4
            }
        );
    }
}
