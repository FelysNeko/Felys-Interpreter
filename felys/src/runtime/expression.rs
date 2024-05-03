use crate::core::frontend::Node;
use crate::core::runtime::{
    Scope,
    Value
};

impl Node {
    pub fn eval(self, env: &Scope) -> Value {
        Value::from(self)
    }
}