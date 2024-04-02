use super::core::Lexer;
use super::core::Token;
use super::core::Node;
use super::core::TokenType as TT;

impl Lexer {
    pub fn scan(r:String) -> Self {
        let mut result = Self { raw: r, data: Vec::new() };
        result
    }

    pub fn parse(&mut self) -> Node {
        Node::from(Token::new(TT::Identifier, 0))
    }
}