mod expression;
mod scanner;
mod core;
mod keyword;

use self::expression::Node;
use self::scanner::Token;
use self::keyword::Statement;

use std::iter::Peekable;
use std::str::Chars;


#[derive(Debug)]
pub struct Program {
    body: Vec<Statement>
}


#[derive(Debug)]
pub struct Lexer<'a> {
    iter: Peekable<Chars<'a>>,
    tokens: Vec<Token>
}
