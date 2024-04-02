pub enum TokenType {
    Identifier
}

pub struct Token {
    pub kind: TokenType,
    pub value: String,
    pub loc: (usize, usize)
}

pub struct Node {
    pub kind: TokenType,
    pub value: String,
    pub branch: Vec<Node>
}

pub struct Parser {
    pub raw: String,
    pub data: Vec<Token>
}
