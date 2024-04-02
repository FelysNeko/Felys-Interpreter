use super::core::Node;
use super::core::Token;
use super::core::TokenType as TT;

impl Node {
    pub fn eval(&mut self) -> Node {
        Self {
            kind: TT::Identifier,
            value: String::from("None"),
            branch: Vec::new()
        }
    }

    pub fn from(tk: Token) -> Self {
        Self {
            kind: tk.kind,
            value: tk.value,
            branch: Vec::new()
        }
    }
}