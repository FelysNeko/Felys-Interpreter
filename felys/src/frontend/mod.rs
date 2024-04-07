mod lexer;
mod token;
mod node;

#[derive(PartialEq, Debug)]
pub enum TokenType {
    Null,
    Identifier,
    Integer,
    String,
    BinaryOperator,
    UnaryOperator,
    OpenParentheses,
    CloseParentheses,
}


pub struct Token {
    pub kind: TokenType,
    pub value: String,
    pub loc: (usize, usize)
}


#[derive(Debug)]
pub struct Node {
    pub kind: TokenType,
    pub value: String,
    pub branch: Vec<Node>
}


pub struct Lexer {
    pub input: String,
    pub tokens: Vec<Token>
}
