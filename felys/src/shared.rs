use std::collections::HashMap;

pub enum KeywordType {
    LET,
    WHILE,
    IF,
    ELIF,
    ELSE,
    RENDER,
    RETURN
}

pub enum ValueType {
    NUMBER,
    STRING,
    IDENT,
    BOOLEAN
}

pub enum NodeType {
    VALUE(ValueType),
    BINOPTR,
    UNAOPTR,
    CALLABLE,
}

pub enum TokenType {
    KEYWORD(KeywordType),
    NODE(NodeType),
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    SEMICOL,
    COMMA,
}

pub struct Token {
    pub ttype: TokenType,
    pub value: String,
}

pub struct Node {
    pub ntype: NodeType,
    pub value: String,
    pub node: Vec<Node>
}

pub struct Statement {
    pub ktype: KeywordType,
    pub expr: Node,
    pub body: Vec<Statement>,
    pub alter: Option<Box<Statement>>
}

pub struct Value {
    pub vtype: ValueType,
    pub value: String
}

pub struct Callable {
    pub param: Vec<String>,
    pub body: Vec<Statement>
}

pub struct Environ {
    pub body: Vec<Scope>
}

pub struct Scope {
    pub variable: HashMap<String, Value>,
    pub callable: HashMap<String, Callable>
}

pub struct Error {
    pub msg: String
}

pub struct Program {
    pub body: Vec<Statement>
}