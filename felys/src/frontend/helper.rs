use super::Node;
use super::Program;
use super::Statement;
use super::Token;
use super::TokenType as TT;

impl Token {
    pub fn null() -> Self {
        Self::new(TT::NULL)
    }

    pub fn identifier() -> Self {
        Self::new(TT::IDENT)
    }

    pub fn number() -> Self {
        Self::new(TT::NUMBER)
    }

    pub fn string() -> Self {
        Self::new(TT::STRING)
    }

    pub fn new(kind: TT) -> Self {
        Self {
            kind,
            value: String::new(),
        }
    }

    pub fn push(&mut self, ch:char) {
        self.value.push(ch);
    }

    pub fn to(&mut self, kind:TT) {
        self.kind = kind;
    }
}


impl Node {
    pub(super) fn from(tk: Token) -> Self {
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

impl Statement {
    pub fn new(keyword: TT, expr: Node,  body: Vec<Statement>) -> Self {
        Self { keyword, expr, body }
    }
}

impl Program {
    pub fn new() -> Self {
        Self { body: Vec::new() }
    }

    pub fn push(&mut self, stat: Statement) {
        self.body.push(stat);
    }
}