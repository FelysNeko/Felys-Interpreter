use std::iter::Peekable;
use std::str::Chars;


#[derive(PartialEq, Clone, Copy, Debug)]
pub enum TokenType {
    NULL,
    IDENT,
    NUMBER,
    STRING,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    SEMICOL,
    BOOLEAN,
    BINOPTR,
    UNAOPTR,
    WHILE,
    IF,
    ELIF,
    ELSE,
    PRINT
}


#[derive(Debug)]
pub struct Lexer<'a> {
    pub iter: Peekable<Chars<'a>>,
    pub tokens: Vec<Token>
}


#[derive(Debug)]
pub struct Token {
    pub kind: TokenType,
    pub value: String,
}


#[derive(Debug)]
pub struct Node {
    pub kind: TokenType,
    pub value: String,
    pub branch: Vec<Node>
}


#[derive(Debug)]
pub struct Statement {
    pub keyword: TokenType,
    pub expr: Node,
    pub body: Vec<Statement>,
    pub alter: Option<Box<Statement>>
}