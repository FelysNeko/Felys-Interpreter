use std::collections::HashMap;

use crate::core::runtime::{
    Scope,
    Value,
};


impl Value {
    
}

impl Scope {
    pub fn new(parent: Option<Box<Scope>>) -> Self {
        Self { variable: HashMap::new(), parent }
    }
}