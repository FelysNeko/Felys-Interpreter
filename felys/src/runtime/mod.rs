mod environ;
mod eval;
use std::collections::HashMap;

#[derive(PartialEq, Debug, Clone)]
pub enum RuntimeType {
    Integer,
    String,
    Bool,
    Null
}

#[derive(Debug, Clone)]
pub struct Value {
    pub kind: RuntimeType,
    pub value: String
}

pub struct Scope {
    pub map: HashMap<String, Value>
}
