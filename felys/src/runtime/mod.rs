mod environ;
mod eval;

use std::collections::HashMap;
use crate::frontend::TokenType as TT;

#[derive(Debug)]
pub struct Value {
    pub kind: TT,
    pub value: String
}

struct Scope {
    map: HashMap<String, Value>
}
