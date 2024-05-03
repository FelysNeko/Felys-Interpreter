use std::process::exit;

use crate::core::frontend::{
    TokenType as TT,
    Node
};
use crate::core::runtime::{
    Scope,
    Value
};

impl Node {
    pub fn eval(self, env: &Scope) -> Value {
        match self.kind {
            TT::STRING |
            TT::NUMBER => Value::from(self),
            _ => {
                println!("cannot eval [{:?}] operation for now", self.kind);
                exit(1);
            }
        }
    }
}