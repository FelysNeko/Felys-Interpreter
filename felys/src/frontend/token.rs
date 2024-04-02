use super::core::Token;
use super::core::TokenType as TT;

impl Token {
    pub fn new(kind: TT, s:usize) -> Self {
        Self {
            kind,
            value: String::new(),
            loc: (s, s)
        }
    }
}