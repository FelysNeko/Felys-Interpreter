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