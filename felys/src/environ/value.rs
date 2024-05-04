use crate::core::Error;
use crate::core::runtime::{
    RuntimeType as RT,
    Value
};
use crate::core::frontend::{
    TokenType as TT,
    Node
};

impl Value {
    pub fn new(kind: RT, value: String) -> Result<Self, Error> {
        Ok(Self { kind, value })
    }

    pub fn from(n: &Node) -> Result<Self, Error> {
        let kind: RT = match n.kind {
            TT::STRING => RT::STRING,
            TT::NUMBER => RT::NUMBER,
            TT::IDENT => RT::IDENT,
            TT::BOOLEAN => RT::BOOLEAN,
            _ => return Error::cvt_to_rt_failed(&n.value)
        };
        Ok(Self { kind, value: n.value.clone() })
    }
}

impl Error {
    fn cvt_to_rt_failed(name: &String) -> Result<Value, Error> {
        Err(Self {
            msg: format!("cannot convert `{}` to runtime type", name)
        })
    }
}