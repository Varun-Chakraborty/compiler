#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Identifier(String),
    Symbol(char),
    Newline,
    Eof,
}

#[derive(Default)]
pub struct TokenStream {
    pub tokens: Vec<Token>,
    pub index: usize,
}

impl TokenStream {
    pub fn new() -> Self {
        Self { tokens: Vec::new(), index: 0 }
    }

    pub fn push(&mut self, token: Token) {
        self.tokens.push(token);
    }

    pub fn seek(&self, at: u32) -> Option<&Token> {
        if at + self.index as u32 >= self.tokens.len() as u32 {
            return None;
        }
        let token = Some(&self.tokens[self.index + at as usize]);
        token
    }

    pub fn seek_as_symbol(&self, at: u32) -> Option<char> {
        let token = self.seek(at);
        match token {
            Some(Token::Symbol(symbol)) => Some(*symbol),
            _ => None,
        }
    }

    pub fn is_eof(&self, at: u32) -> bool {
        let token = self.seek(at);
        match token {
            Some(Token::Eof) => true,
            _ => false,
        }
    }

    pub fn next(&mut self) {
        self.index += 1;
    }
}
