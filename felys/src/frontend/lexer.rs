use super::core::Lexer;
use super::core::Token;
use super::core::Node;
use super::core::TokenType as tkT;

impl Lexer {
    pub fn new(r:String) -> Self {
        let mut result = Self { raw: r, data: Vec::new() };
        result.scan();
        result
    }

    pub fn scan(&mut self) {

    }

    pub fn parse(&mut self) -> Node {
        Node::from(Token::new(tkT::Identifier, 0))
    }
}