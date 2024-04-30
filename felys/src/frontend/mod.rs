mod lexer;
mod more;

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


pub(super) struct Token {
    kind: TokenType,
    value: String,
    loc: (usize, usize)
}


#[derive(Debug)]
pub struct Node {
    pub kind: TokenType,
    pub value: String,
    pub branch: Vec<Node>
}


pub struct Lexer {
    input: String,
    tokens: Vec<Token>
}
