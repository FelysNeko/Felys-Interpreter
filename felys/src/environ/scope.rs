use std::collections::HashMap;

use crate::core::Error;
use crate::core::runtime::{
    RuntimeType as RT,
    Scope,
    Value
};

impl Scope {
    pub fn init() -> Self {
        let mut env: HashMap<String, Value> = HashMap::new();
        env.insert(
            "elysia".to_string(),
            Value { kind:RT::STRING, value:"粉色妖精小姐♪".to_string() }
        );
        env.insert(
            "ELYSIA".to_string(),
            Value { kind:RT::STRING, value:"「真我·人之律者」".to_string() }
        );
        Self { data: vec![env] }
    }

    pub fn set(&mut self, name: &String, val: &Value) -> Result<(), Error> {
        for scope in self.data.iter_mut().rev() {
            if scope.contains_key(name) {
                scope.insert(name.clone(), val.clone());
                return Ok(());
            }
        }

        if let Some(scope) = self.data.last_mut() {
            scope.insert(name.clone(), val.clone());
            Ok(())
        } else {
            Error::no_scope_exist()
        }
    }

    pub fn get(&mut self, name: &String) -> Result<Value, Error> {
        for scope in self.data.iter().rev() {
            if let Some(val) = scope.get(name) {
                return Ok(val.clone());
            }
        }
        Error::var_not_exist(name)
    }

    pub fn extend(&mut self) {
        self.data.push(HashMap::new());
    }

    pub fn shrink(&mut self) {
        self.data.pop();
    }
}


impl Error {
    fn var_not_exist(name: &String) -> Result<Value, Error> {
        Err(Self {
            msg: format!("`{}` does not exist", name)
        })
    }

    fn no_scope_exist() -> Result<(), Error> {
        Err(Self {
            msg: format!("no scope exist")
        })
    }
}