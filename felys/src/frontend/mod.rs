mod parser;
mod scanner;
mod helper;

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

    // keyword
    WHILE,
    IF,
    ELIF,
    ELSE,

    // boolean
    TRUE,
    FALSE,

    // binary operator
    ADD,
    SUB,
    MUL,
    DIV,
    MOD,
    LGR,
    SMR,
    LEQ,
    SEQ,
    ASN,
    EQ,
    NE,
    AND,
    OR,

    // unary operator
    POS,
    NEG,
    NOT,
}

#[derive(Debug)]
struct Token {
    kind: TokenType,
    value: String,
}

#[derive(Debug)]
pub struct Statement {
    pub keyword: TokenType,
    pub expr: Node,
    pub body: Vec<Statement>
}


#[derive(Debug)]
pub struct Node {
    pub kind: TokenType,
    pub value: String,
    pub branch: Vec<Node>
}

#[derive(Debug)]
pub struct Lexer<'a> {
    iter: Peekable<Chars<'a>>,
    tokens: Vec<Token>
}

impl Lexer<'_> {
    pub fn parse(input: String) -> Option<Statement> {
        let mut lxr: Lexer<'_> = Lexer {
            iter: input.chars().peekable(),
            tokens: Vec::new()
        };
        lxr._scan();
        lxr._parse()
    }
}