use std::collections::HashMap;

use crate::core::runtime::Scope;

impl Scope {
    pub fn new(parent: Option<Box<Scope>>) -> Self {
        Self { variable: HashMap::new(), parent }
    }
}