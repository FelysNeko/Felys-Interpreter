use std::collections::HashMap;


#[derive(Debug)]
pub enum RuntimeType {
    Number,
    String,
    Bool,
    Null
}


#[derive(Debug)]
pub struct Value {
    pub kind: RuntimeType,
    pub value: String
}


#[derive(Debug)]
pub struct Scope {
    pub variable: HashMap<String, Value>,
    pub parent: Box<Scope>
}