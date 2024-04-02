use super::core::Token;
use super::core::TokenType as tkT;

impl Token {
    pub fn new(kind: tkT, s:usize) -> Self {
        Self {
            kind,
            value: String::new(),
            loc: (s, s)
        }
    }
}