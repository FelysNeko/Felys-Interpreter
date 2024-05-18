use std::collections::HashMap;
use crate::shared::{
    ValueType as VT,
    Statement,
    Callable,
    Environ,
    Output,
    Value,
    Scope,
    Error
};


impl Output {
    pub(super) fn new() -> Self {
        Self { body: Vec::new() }
    }
}


impl Value {
    pub(super) fn new(vtype: VT, value: String) -> Result<Self, Error> {
        Ok(Self { vtype, value })
    }

    pub(super) fn parse_to_f64(&self) -> Result<f64, Error> {
        if self.vtype == VT::NUMBER {
            match self.value.parse::<f64>() {
                Ok(num) => Ok(num),
                Err(_) => Error::cannot_parse_number(&self.value)
            }
        } else {
            Error::cannot_parse_number(&self.value)
        }
    }
    
    pub(super) fn parse_to_bool(&self) -> Result<bool, Error> {
        match self.vtype {
            VT::BOOLEAN => Ok(if self.value.as_str() == "true" { true } else { false }),
            VT::STRING => Ok(self.value.len() != 0),
            VT::NUMBER => Ok(self.parse_to_f64()? != 0.0),
            _ => Error::cannot_parse_bool(&self.value)
        }
    }
}


impl Environ {
    pub(super) fn new(args: Vec<(String, Value)>) -> Self {
        Self { body: vec![Scope::new(args)] }
    }
}


impl Scope {
    pub(super) fn new(params: Vec<(String, Value)>) -> Self {
        let mut variable: HashMap<String, Value> = HashMap::new();
        for (k, v) in params {
            variable.insert(k, v);
        }
        Self {
            variable,
            callable: HashMap::new()
        }
    }
}


impl Callable {
    pub(super) fn new(args: Vec<String>, body: Vec<Statement>) -> Self {
        Self { args, body }
    }
}


impl Error {
    fn cannot_parse_number(value: &String) -> Result<f64, Error> {
        Err(Self {
            msg: format!("cannot parse `{}` to number", value)
        })
    }

    fn cannot_parse_bool(value: &String) -> Result<bool, Error> {
        Err(Self {
            msg: format!("cannot parse `{}` to bool", value)
        })
    }
}
