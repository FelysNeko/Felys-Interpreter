use std::collections::HashMap;
use std::process::exit;

use crate::core::runtime::{
    Scope,
    Value
};

impl Scope {
    pub fn init() -> Self {
        Self { data: vec![HashMap::new()] }
    }

    pub fn set(&mut self, name: &String, val: &Value) {
        for scope in self.data.iter_mut().rev() {
            if scope.contains_key(name) {
                scope.insert(name.clone(), val.clone());
                return;
            }
        }

        if let Some(scope) = self.data.last_mut() {
            scope.insert(name.clone(), val.clone());
        } else {
            println!("no scope exists");
            exit(1);
        }
    }

    pub fn get(&mut self, name: &String) -> Value {
        for scope in self.data.iter().rev() {
            if let Some(val) = scope.get(name) {
                return val.clone();
            }
        }
        println!("no var called [{}]", name);
        exit(1);
    }

    pub fn extend(&mut self) {
        self.data.push(HashMap::new());
    }

    pub fn shrink(&mut self) {
        self.data.pop();
    }
}