use std::{collections::HashMap, process::exit};

use crate::core::runtime::{Scope, Value};

impl Scope {
    pub fn new() -> Self {
        Self { variable: HashMap::new(), parent: None }
    }

    pub fn set(&mut self, name: String, val: Value) {
        self.variable.insert(name, val);
    }

    pub fn get(&mut self, name: String) -> Value {
        match self.variable.get(&name) {
            Some(val) => val.clone(),
            None => {
                println!("no var called [{}]", name);
                exit(1)
            }
        }
    }
}