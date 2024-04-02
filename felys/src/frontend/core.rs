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


pub struct Lexer {
    pub raw: String,
    pub data: Vec<Token>
}


pub fn eval(input: String) -> Node {
    let mut lexer: Lexer = Lexer::new(input);
    let mut entry: Node = lexer.parse();
    entry.eval()
}