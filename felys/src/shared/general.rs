use super::{
    TokenType as TT,
    NodeType as NT,
    ValueType as VT,
    Token
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
