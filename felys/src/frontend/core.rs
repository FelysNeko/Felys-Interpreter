#[derive(PartialEq)]
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


pub struct Node {
    pub kind: TokenType,
    pub value: String,
    pub branch: Vec<Node>
}


pub struct Lexer {
    pub input: String,
    pub tokens: Vec<Token>
}


pub fn eval(input: String) -> Node {
    let mut lexer: Lexer = Lexer::scan(input);
    let mut entry: Node = lexer.parse();
    entry.eval()
}