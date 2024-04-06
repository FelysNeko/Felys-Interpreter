use super::core::Node;
use super::core::Token;

impl Node {
    pub fn from(tk: Token) -> Self {
        Self {
            kind: tk.kind,
            value: tk.value,
            branch: Vec::new()
        }
    }

    pub fn push(&mut self, n:Node) {
        self.branch.push(n)
    }
}