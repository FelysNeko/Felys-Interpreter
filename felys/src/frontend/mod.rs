mod core;
mod token;
mod expression;
mod statement;

pub use core::parse;

use crate::shared::Token;

use std::iter::Peekable;
use std::str::Chars;

struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
    token: Vec<Token>
}