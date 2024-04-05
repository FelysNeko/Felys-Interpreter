use super::core::Token;
use super::core::TokenType as TT;
use colored::Colorize;
use std::fmt;

impl fmt::Debug for TT {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            TT::Null => write!(f, "{}", "NUL".red().bold()),
            TT::Identifier => write!(f, "IDT"),
            TT::Integer => write!(f, "INT"),
            TT::String => write!(f, "STR"),
            TT::UnaryOperator => write!(f, "UOP"),
            TT::BinaryOperator => write!(f, "BOP"),
            TT::OpenParentheses => write!(f, "OPA"),
            TT::CloseParentheses => write!(f, "CPA"),
        }
    }
}


impl Token {
    pub fn new(kind: TT, s:usize) -> Self {
        Self {
            kind,
            value: String::new(),
            loc: (s, s)
        }
    }

    pub fn push(&mut self, ch:char) {
        self.value.push(ch);
        self.loc.1 += 1;
    }
}