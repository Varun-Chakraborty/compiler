pub mod token;

use self::token::{Token, TokenStream};
use std::mem;

#[derive(Debug, thiserror::Error)]
pub enum LexerError {}

pub struct Lexer {
    tokens: TokenStream,
    token: String,
}

impl Lexer {
    pub fn new() -> Self {
        Self {
            tokens: TokenStream::new(),
            token: String::new(),
        }
    }

    pub fn push_identifier(&mut self) {
        if self.token.is_empty() {
            return;
        }
        self.tokens.push(Token::Identifier(mem::take(&mut self.token)));
    }

    pub fn push_symbol(&mut self, ch: char) {
        self.tokens.push(Token::Symbol(ch));
    }

    pub fn push_newline(&mut self) {
        self.tokens.push(Token::Newline);
    }

    pub fn push_eof(&mut self) {
        self.tokens.push(Token::Eof);
    }

    pub fn lex(&mut self, assembly_program: &str) -> Result<TokenStream, LexerError> {
        let mut is_comment = false;
        for char in assembly_program.chars() {
            if is_comment {
                if char == '\n' {
                    is_comment = false;
                    self.push_newline();
                }
                continue;
            }
            match char {
                ':' | ',' => {
                    self.push_identifier();
                    self.push_symbol(char);
                }
                ';' => {
                    is_comment = true;
                }
                ' ' | '\t' => {
                    self.push_identifier();
                }
                '\n' => {
                    self.push_identifier();
                    self.push_newline();
                }
                _ => {
                    self.token.push(char);
                }
            }
        }
        self.push_identifier();
        self.push_eof();
        Ok(mem::take(&mut self.tokens))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        let mut lexer = Lexer::new();
        let tokens = lexer.lex("MOVER R1, 0").unwrap().tokens;

        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens[0], Token::Identifier("MOVER".to_string()));
        assert_eq!(tokens[1], Token::Identifier("R1".to_string()));
        assert_eq!(tokens[2], Token::Symbol(','));
        assert_eq!(tokens[3], Token::Identifier("0".to_string()));
        assert_eq!(tokens[4], Token::Eof);
    }
}
