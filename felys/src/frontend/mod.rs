mod expression;
mod scanner;
mod helper;
mod statement;

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
pub struct Token {
    pub kind: TokenType,
    pub value: String,
}

#[derive(Debug)]
pub struct Program {
    pub body: Vec<Statement>
}

#[derive(Debug)]
pub struct Statement {
    pub keyword: Option<TokenType>,
    pub expr: Option<Node>,
    pub body: Option<Vec<Statement>>,
    pub alter: Option<Box<Statement>>
}

#[derive(Debug)]
pub struct Node {
    pub kind: TokenType,
    pub value: String,
    pub branch: Vec<Node>
}

#[derive(Debug)]
pub struct Lexer<'a> {
    pub iter: Peekable<Chars<'a>>,
    pub tokens: Vec<Token>
}

impl Lexer<'_> {
    pub fn parse(input: String) -> Program {
        let mut lxr: Lexer<'_> = Lexer {
            iter: input.chars().peekable(),
            tokens: Vec::new()
        };

        while let Some(tk) = lxr.scan_next() {
            lxr.tokens.push(tk);
        };

        // we want to scan front left to right
        // but `pop()` get you the last element
        // so `reverse()` everything first
        lxr.tokens.reverse();

        let mut main: Program = Program::new();
        while let Some(stat) = lxr.parse_next() {
            main.push(stat);
        }
        main
    }
}