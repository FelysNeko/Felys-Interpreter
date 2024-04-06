use super::Node;
use super::Token;
use super::TokenType as TT;

impl Node {
    pub fn from(tk: Token) -> Self {
        Self {
            kind: tk.kind,
            value: tk.value,
            branch: Vec::new()
        }
    }

    pub fn new(kind: TT, value:String) -> Self {
        Self {
            kind,
            value,
            branch: Vec::new()
        }
    }

    pub fn push(&mut self, n:Node) {
        self.branch.push(n)
    }

    pub fn null() -> Node {
        Node::new(TT::Null, String::from(""))
    }
}