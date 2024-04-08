use crate::error::{Handler, Msg};
use crate::error::Handler::{Good, Bad};

use super::Scope;
use super::Value;
use super::RuntimeType as RT;
use std::collections::HashMap;


impl Scope {
    pub fn new() -> Self {
        let mut env = Self { map: HashMap::new() };
        env.assign(
            String::from("true"),
            Value::new(RT::Bool, String::from("true"))
        );
        env.assign(
            String::from("false"),
            Value::new(RT::Bool, String::from("false"))
        );
        env.map.insert(
            String::from("elysia"),
            Value::new(RT::String, String::from("嗨～想我了吗？"))
        );
        env
    }

    pub fn get(&self, name:String) -> Handler<Value, String> {
        if let Some(val) = self.map.get(&name) {
            Good(val.clone())
        } else {
            Bad(Msg::variable_not_defined(name))
        }
    }

    pub fn assign(&mut self, k:String, v:Value) {
        if k.as_str() != "elysia" {
            self.map.insert(k, v);
        }
    }
}