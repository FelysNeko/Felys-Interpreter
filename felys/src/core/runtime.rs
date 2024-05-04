use std::collections::HashMap;


#[derive(PartialEq, Debug)]
pub enum RuntimeType {
    NUMBER,
    STRING,
    IDENT,
    BOOLEAN
}


#[derive(Debug)]
pub struct Value {
    pub kind: RuntimeType,
    pub value: String
}


#[derive(Debug)]
pub struct Scope {
    pub variable: HashMap<String, Value>,
    pub parent: Option<Box<Scope>>
}