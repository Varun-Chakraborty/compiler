use std::mem;

use crate::lexer::token::Token;

use super::{super::lexer::token::TokenStream, instruction::Statement};

#[derive(Debug, thiserror::Error)]
pub enum SyntacticError {
    #[error("Parsing error: {message}")]
    ParseError { message: String },
    #[error("Opcode is missing at line: {line}")]
    OpcodeMissing { line: String },
}

#[derive(PartialEq, Debug)]
enum DFAState {
    Start,
    AfterLabel,
    AfterOpcode,
    AfterOperand,
    ExpectOperand, // after comma
}

pub struct SyntacticParser {
    statements: Vec<Statement>,
}

impl SyntacticParser {
    pub fn new() -> Self {
        return Self { statements: vec![] };
    }

    pub fn parse(&mut self, mut tokens: TokenStream) -> Result<Vec<Statement>, SyntacticError> {
        let mut statement = Statement::new();
        let mut state = DFAState::Start;
        while !tokens.is_eof(0) {
            let current_token = tokens.seek(0);
            match current_token {
                Some(Token::Identifier(identifier)) => {
                    match state {
                        DFAState::Start => {
                            // label or opcode
                            if let Some(':') = tokens.seek_as_symbol(1) {
                                statement.set_label(identifier.clone());
                                tokens.next();
                                tokens.next();
                                state = DFAState::AfterLabel;
                            } else {
                                statement.set_operation_name(identifier.clone());
                                tokens.next();
                                state = DFAState::AfterOpcode;
                            }
                        }
                        DFAState::AfterLabel => {
                            statement.set_operation_name(identifier.clone());
                            tokens.next();
                            state = DFAState::AfterOpcode;
                        }
                        DFAState::ExpectOperand | DFAState::AfterOpcode => {
                            statement.add_operand(identifier.clone());
                            tokens.next();
                            state = DFAState::AfterOperand;
                        }
                        _ => {
                            return Err(SyntacticError::ParseError {
                                message: format!("Unexpected identifier: {}", identifier),
                            });
                        }
                    }
                }
                Some(Token::Symbol(s)) => match state {
                    DFAState::AfterOperand => {
                        if *s == ',' {
                            state = DFAState::ExpectOperand;
                            tokens.next();
                        } else {
                            return Err(SyntacticError::ParseError {
                                message: format!("Unexpected symbol: {}", s),
                            });
                        }
                    }
                    _ => {
                        return Err(SyntacticError::ParseError {
                            message: format!("Unexpected symbol: {}", s),
                        });
                    }
                },
                Some(Token::Newline) => {
                    if state == DFAState::ExpectOperand {
                        return Err(SyntacticError::ParseError {
                            message: format!("An identifier is expected after comma"),
                        });
                    }
                    if state != DFAState::Start {
                        self.statements.push(statement);
                    }
                    statement = Statement::new();
                    tokens.next();
                    state = DFAState::Start;
                }
                _ => {}
            }
        }
        if state == DFAState::ExpectOperand {
            return Err(SyntacticError::ParseError {
                message: format!("An identifier is expected after comma"),
            });
        } else if state != DFAState::Start {
            self.statements.push(statement);
        }
        return Ok(mem::take(&mut self.statements));
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::lexer::token::{Token, TokenStream};
    use super::*;

    #[test]
    fn test_basic_parsing() {
        let mut parser = SyntacticParser::new();
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
        let statements = parser.parse(tokens).unwrap();
        assert_eq!(statements.len(), 2);
        assert_eq!(statements[0].label, Some("MOVE".to_string()));
        assert_eq!(statements[0].operation_name, Some("MOVER".to_string()));
        assert_eq!(
            statements[0].operands,
            Some(vec!["R0".to_string(), "0".to_string()])
        );
        assert_eq!(statements[1].label, Some("MOVE1".to_string()));
        assert_eq!(statements[1].operation_name, Some("MOVER".to_string()));
        assert_eq!(
            statements[1].operands,
            Some(vec!["R0".to_string(), "0".to_string()])
        );
    }

    #[test]
    fn test_single_operand() {
        let mut parser = SyntacticParser::new();
        let mut tokens = TokenStream::new();
        tokens.push(Token::Identifier("CALL".to_string()));
        tokens.push(Token::Identifier("R0".to_string()));
        tokens.push(Token::Eof);
        let statements = parser.parse(tokens).unwrap();
        assert_eq!(statements.len(), 1);
        assert_eq!(statements[0].label, None);
        assert_eq!(statements[0].operation_name, Some("CALL".to_string()));
        assert_eq!(statements[0].operands, Some(vec!["R0".to_string()]));
    }

    #[test]
    fn test_no_operand() {
        let mut parser = SyntacticParser::new();
        let mut tokens = TokenStream::new();
        tokens.push(Token::Identifier("RET".to_string()));
        tokens.push(Token::Eof);
        let statements = parser.parse(tokens).unwrap();
        assert_eq!(statements.len(), 1);
        assert_eq!(statements[0].label, None);
        assert_eq!(statements[0].operation_name, Some("RET".to_string()));
        assert_eq!(statements[0].operands, None);
    }

    #[test]
    fn test_unusual_statement() {
        let mut parser = SyntacticParser::new();
        let mut tokens = TokenStream::new();
        tokens.push(Token::Identifier("MOVE".to_string()));
        tokens.push(Token::Symbol(':'));
        tokens.push(Token::Symbol(':'));
        tokens.push(Token::Eof);
        
        // should fail
        let statements = parser.parse(tokens);
        assert!(statements.is_err());
    }
}
