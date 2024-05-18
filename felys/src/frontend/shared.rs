use crate::shared::{
    KeywordType as KT,
    TokenType as TT,
    Statement,
    Program,
    Token,
    Error,
    Node
};
use super::Lexer;


impl Node {
    pub(crate) fn from(tk: Token) -> Result<Self, Error> {
        if let TT::NODE(ntype) = tk.ttype {
            Ok(Self { ntype, value: tk.value, nodes: Vec::new() })
        } else {
            Error::invalid_token_type(tk.ttype)
        }
    }
}


impl Lexer<'_> {
    pub(super) fn eat(&mut self, t: TT) -> Result<(), Error>{
        if let Some(tk) = self.token.pop() {
            if tk.ttype != t {
                Error::incorrect_next_token(t, tk.value)
            } else {
                Ok(())
            }
        } else {
            Error::nothing_to_eat(t)
        }
    }
}


impl Statement {
    pub(super) fn new(
        ktype: KT,
        expr: Node,
        body: Vec<Statement>,
        alter: Option<Box<Statement>>
    ) -> Result<Self, Error> {
        Ok(Self { ktype, expr, body, alter })
    }
}


impl Program {
    pub fn new() -> Self {
        Self { body: Vec::new() }
    }
}


impl Error {
    fn invalid_token_type(t: TT) -> Result<Node, Error> {
        Err(Self { msg: format!("cannot convert token {:?} to node", t) })
    }

    fn incorrect_next_token(e: TT, s: String) -> Result<(), Error> {
        Err(Self { msg: format!("expect `{:?}`, but see `{}`", e, s) })
    }

    fn nothing_to_eat(e: TT) -> Result<(), Error> {
        Err(Self { msg: format!("expect `{:?}`, but nothing to eat", e) })
    }
}