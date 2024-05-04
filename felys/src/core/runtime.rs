use std::collections::HashMap;


#[derive(PartialEq, Clone, Debug)]
pub enum RuntimeType {
    NUMBER,
    STRING,
    IDENT,
    BOOLEAN
}


#[derive(Debug, Clone)]
pub struct Value {
    pub kind: RuntimeType,
    pub value: String
}


#[derive(Debug)]
pub struct Scope {
    pub data: Vec<HashMap<String, Value>>,
}

#[derive(Debug)]
pub struct Output {
    pub body: Vec<String>
}