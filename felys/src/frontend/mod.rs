mod core;
mod token;
mod shared;
mod expression;
mod statement;

pub use core::parse;

use std::iter::Peekable;
use std::str::Chars;
use crate::shared::Token;

struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
    token: Vec<Token>
}