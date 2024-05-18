use std::collections::HashMap;

use super::{
    TokenType as TT,
    NodeType as NT,
    ValueType as VT,
    Program,
    Output,
    Environ,
    Value,
    Token,
    Scope
};


impl From<TT> for Token {
    fn from(ttype: TT) -> Self {
        Self { ttype, value: String::new() }
    }
}

impl From<NT> for Token {
    fn from(ntype: NT) -> Self {
        Self { ttype: TT::NODE(ntype), value: String::new() }
    }
}

impl From<VT> for Token {
    fn from(vtype: VT) -> Self {
        Self { ttype: TT::NODE(NT::VALUE(vtype)), value: String::new() }
    }
}


impl Program {
    pub fn new() -> Self {
        Self { body: Vec::new() }
    }
}


impl Environ {
    pub fn new(args: Vec<(String, Value)>) -> Self {
        Self { body: vec![Scope::new(args)] }
    }
}


impl Scope {
    pub fn new(params: Vec<(String, Value)>) -> Self {
        let mut variable: HashMap<String, Value> = HashMap::new();
        for (k, v) in params {
            variable.insert(k, v);
        }
        Self {
            variable,
            callable: HashMap::new()
        }
    }
}


impl Output {
    pub fn new() -> Self {
        Self { lines: Vec::new() }
    }

    pub fn render(&self) -> String {
        return self.lines.join("\n")
    }
}
