mod environ;
mod eval;
use std::collections::HashMap;

#[derive(PartialEq, Debug)]
pub enum RuntimeType {
    Integer,
    String,
    Bool,
    Null
}

#[derive(Debug)]
pub struct Value {
    pub kind: RuntimeType,
    pub value: String
}

struct Scope {
    map: HashMap<String, Value>
}
