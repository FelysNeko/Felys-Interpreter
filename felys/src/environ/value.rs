use std::process::exit;

use crate::core::runtime::{
    RuntimeType as RT,
    Value
};
use crate::core::frontend::{
    TokenType as TT,
    Node
};

impl Value {
    pub fn from(n: Node) -> Self {
        let kind: RT = match n.kind {
            TT::STRING => RT::STRING,
            TT::NUMBER => RT::NUMBER,
            TT::IDENT => RT::IDENT,
            TT::TRUE => RT::TRUE,
            TT::FALSE => RT::FALSE,
            _ => {
                println!("cannot convert [{:?}] to runtime type", n.kind);
                exit(1)
            }
        };
        Self { kind, value: n.value }
    }
}