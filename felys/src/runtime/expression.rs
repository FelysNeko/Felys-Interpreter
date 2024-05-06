use crate::core::Error;
use crate::core::frontend::{
    TokenType as TT,
    Node
};
use crate::core::runtime::{
    RuntimeType as RT,
    Scope,
    Value
};


impl Node {
    pub fn eval(&mut self, env: &mut Scope) -> Result<Value, Error> {
        match self.kind {
            TT::STRING |
            TT::NUMBER |
            TT::BOOLEAN => Value::from(self),
            TT::IDENT => env.get(&self.value),
            TT::BINOPTR => self._eval_binoptr(env),
            TT::UNAOPTR => self._eval_unaoptr(env),
            _ => Error::cannot_eval_token(&self.value)
        }
    }

    fn _eval_unaoptr(&mut self, env: &mut Scope) -> Result<Value, Error> {
        let val: Value = match self.branch.first_mut() {
            Some(node) => node.eval(env)?,
            None => Error::node_branches_missing(&self.value)?
        };

        match self.value.as_str() {
            "+" => val._pos(),
            "-" => val._neg(),
            "!" => val._not(),
            _ => Error::unaoptr_not_impl(&self.value)
        }
    }

    fn _eval_binoptr(&mut self, env: &mut Scope) -> Result<Value, Error> {
        let rval: Value = match self.branch.last_mut() {
            Some(node) => node.eval(env)?,
            None => Error::node_branches_missing(&self.value)?
        };

        let lval: Value = match self.branch.first_mut() {
            Some(node) => if self.value.as_str() == "=" {
                if node.kind == TT::IDENT {
                    env.set(&node.value, &rval)?;
                    return Ok(rval);
                } else {
                    Error::cannot_assign(&self.value)?
                }
            } else {
                node.eval(env)?
            },
            None => Error::node_branches_missing(&self.value)?
        };

        match self.value.as_str() {
            "+" => lval._add(rval),
            "-" => lval._sub(rval),
            "*" => lval._mul(rval),
            "/" => lval._div(rval),
            "%" => lval._mod(rval),
            ">" => lval._lgr(rval),
            "<" => lval._smr(rval),
            "==" => lval._eq(rval),
            "!=" => lval._ne(rval),
            ">=" => lval._leq(rval),
            "<=" => lval._seq(rval),
            "and" => lval._and(rval),
            "or" => lval._or(rval),
            _ => Error::binoptr_not_impl(&self.value)
        }
    }
}


impl Value {
    fn _parse_to_f64(&self) -> Result<f64, Error> {
        if self.kind == RT::NUMBER {
            match self.value.parse::<f64>() {
                Ok(num) => Ok(num),
                Err(_) => Error::cannot_parse_number(&self.value)
            }
        } else {
            Error::cannot_parse_number(&self.value)
        }
    }

    pub fn _parse_to_bool(&self) -> Result<bool, Error> {
        match self.kind {
            RT::BOOLEAN => Ok(if self.value.as_str() == "true" { true } else { false }),
            RT::STRING => Ok(self.value.len() != 0),
            RT::NUMBER => Ok(self._parse_to_f64()? != 0.0),
            _ => Error::cannot_parse_bool(&self.value)
        }
    }

    fn _add(&self, rval: Value) -> Result<Value, Error> {
        let val: String = match (&self.kind, &rval.kind) {
            (RT::STRING, _) |
            (_, RT::STRING) => self.value.clone() + &rval.value.clone(),
            _ => (self._parse_to_f64()? + rval._parse_to_f64()?).to_string()
        };
        Value::new(RT::NUMBER, val)
    }

    fn _sub(&self, rval: Value) -> Result<Value, Error> {
        let val: f64 = self._parse_to_f64()? - rval._parse_to_f64()?;
        Value::new(RT::NUMBER, val.to_string())
    }

    fn _mul(&self, rval: Value) -> Result<Value, Error> {
        let val: f64 = self._parse_to_f64()? * rval._parse_to_f64()?;
        Value::new(RT::NUMBER, val.to_string())
    }

    fn _div(&self, rval: Value) -> Result<Value, Error> {
        let val: f64 = self._parse_to_f64()? / rval._parse_to_f64()?;
        Value::new(RT::NUMBER, val.to_string())
    }

    fn _mod(&self, rval: Value) -> Result<Value, Error> {
        let val: f64 = self._parse_to_f64()? % rval._parse_to_f64()?;
        Value::new(RT::NUMBER, val.to_string())
    }

    fn _eq(&self, rval: Value) -> Result<Value, Error> {
        let val: bool = self.value == rval.value;
        Value::new(RT::BOOLEAN, val.to_string())
    }

    fn _ne(&self, rval: Value) -> Result<Value, Error> {
        let val: bool = self.value != rval.value;
        Value::new(RT::BOOLEAN, val.to_string())
    }

    fn _lgr(&self, rval: Value) -> Result<Value, Error> {
        let val: bool = self._parse_to_f64()? > rval._parse_to_f64()?;
        Value::new(RT::BOOLEAN, val.to_string())
    }

    fn _leq(&self, rval: Value) -> Result<Value, Error> {
        let val: bool = self._parse_to_f64()? >= rval._parse_to_f64()?;
        Value::new(RT::BOOLEAN, val.to_string())
    }

    fn _smr(&self, rval: Value) -> Result<Value, Error> {
        let val: bool = self._parse_to_f64()? < rval._parse_to_f64()?;
        Value::new(RT::BOOLEAN, val.to_string())
    }

    fn _seq(&self, rval: Value) -> Result<Value, Error> {
        let val: bool = self._parse_to_f64()? <= rval._parse_to_f64()?;
        Value::new(RT::BOOLEAN, val.to_string())
    }

    fn  _and(&self, rval: Value) -> Result<Value, Error> {
        if self._parse_to_bool()? && rval._parse_to_bool()? {
            Ok(rval)
        } else {
            Value::new(RT::BOOLEAN, false.to_string())
        }
    }

    fn _or(&self, rval: Value) -> Result<Value, Error> {
        if self._parse_to_bool()? {
            Ok(self.clone())
        } else if self._parse_to_bool()? {
            Ok(rval)
        } else {
            Value::new(RT::BOOLEAN, false.to_string())
        }
    }

    fn _pos(&self) -> Result<Value, Error> {
        let val: f64 = self._parse_to_f64()?;
        Value::new(RT::NUMBER, val.to_string())
    }

    fn _neg(&self) -> Result<Value, Error> {
        let val: f64 = -self._parse_to_f64()?;
        Value::new(RT::NUMBER, val.to_string())
    }

    fn _not(&self) -> Result<Value, Error> {
        let val: bool = !self._parse_to_bool()?;
        Value::new(RT::BOOLEAN, val.to_string())
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

    fn node_branches_missing(value: &String) -> Result<Value, Error> {
        Err(Self {
            msg: format!("node `{}` misses required branches", value)
        })
    }

    fn unaoptr_not_impl(value: &String) -> Result<Value, Error> {
        Err(Self {
            msg: format!("unary operator `{}` is not supported", value)
        })
    }

    fn binoptr_not_impl(value: &String) -> Result<Value, Error> {
        Err(Self {
            msg: format!("binary operator `{}` is not supported", value)
        })
    }

    fn cannot_eval_token(value: &String) -> Result<Value, Error> {
        Err(Self {
            msg: format!("token `{}` is not supported", value)
        })
    }

    fn cannot_assign(value: &String) -> Result<Value, Error> {
        Err(Self {
            msg: format!("cannot assign to token `{}`", value)
        })
    }
}