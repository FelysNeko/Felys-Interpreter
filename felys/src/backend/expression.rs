use crate::shared::{
    ValueType as VT,
    NodeType as NT,
    Environ,
    Value,
    Error,
    Node
};


impl Node {
    pub(super) fn eval(&self, env: &mut Environ) -> Result<Value, Error> {
        match self.ntype {
            NT::VALUE(VT::IDENT) => env.get(&self.value),
            NT::VALUE(_) => todo!(),
            NT::BINOPTR => todo!(),
            NT::UNAOPTR => todo!(),
            NT::CALLABLE => todo!(),
        }
    }
}

impl Value {
    fn parse_to_f64(&self) -> Result<f64, Error> {
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