mod core;
mod token;
mod expression;
mod statement;

pub use core::parse;

use crate::shared::Token;

use std::iter::Peekable;
use std::str::Chars;

pub struct Lexer<'a> {
    pub chars: Peekable<Chars<'a>>,
    pub token: Vec<Token>
}