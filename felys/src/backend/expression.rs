use crate::shared::{
    ValueType as VT,
    NodeType as NT,
    Environ,
    Output,
    Value,
    Error,
    Node
};


impl Node {
    pub(super) fn eval(&self, env: &mut Environ, out: &mut Output) -> Result<Value, Error> {
        match self.ntype {
            NT::VALUE(_) => self.to_value(env),
            NT::BINOPTR => self.eval_binoptr(env, out),
            NT::UNAOPTR => self.eval_unaoptr(env, out),
            NT::CALLABLE => todo!(),
        }
    }

    fn to_value(&self, env: &mut Environ) -> Result<Value, Error> {
        if let NT::VALUE(vtype) = self.ntype {
            if vtype == VT::IDENT {
                env.get(&self.value)
            } else {
                Ok(Value { vtype, value: self.value.clone() })
            }
        } else {
            Error::cvt_to_vt_failed(&self.value)
        }
    }

    fn eval_binoptr(&self, env: &mut Environ, out: &mut Output) -> Result<Value, Error> {
        let rval: Value = match self.nodes.last() {
            Some(node) => node.eval(env, out)?,
            None => Error::node_branches_missing(&self.value)?
        };

        let lval: Value = match self.nodes.first() {
            Some(node) => if self.value.as_str() == "=" {
                if node.ntype == NT::VALUE(VT::IDENT) {
                    env.assign(&node.value, rval.clone())?;
                    return Ok(rval);
                } else {
                    Error::cannot_assign(&self.value)?
                }
            } else {
                node.eval(env, out)?
            },
            None => Error::node_branches_missing(&self.value)?
        };

        match self.value.as_str() {
            "+" => lval.add(rval),
            "-" => lval.sub(rval),
            "*" => lval.mul(rval),
            "/" => lval.div(rval),
            "%" => lval.rem(rval),
            ">" => lval.lgr(rval),
            "<" => lval.smr(rval),
            "==" => lval.eq(rval),
            "!=" => lval.ne(rval),
            ">=" => lval.leq(rval),
            "<=" => lval.seq(rval),
            "and" => lval.and(rval),
            "or" => lval.or(rval),
            _ => Error::binoptr_not_impl(&self.value)
        }
    }

    fn eval_unaoptr(&self, env: &mut Environ, out: &mut Output) -> Result<Value, Error> {
        let val: Value = match self.nodes.first() {
            Some(node) => node.eval(env, out)?,
            None => Error::node_branches_missing(&self.value)?
        };

        match self.value.as_str() {
            "+" => val.pos(),
            "-" => val.neg(),
            "!" => val.not(),
            _ => Error::unaoptr_not_impl(&self.value)
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

    fn pos(&self) -> Result<Value, Error> {
        let val: f64 = self.parse_to_f64()?;
        Value::new(VT::NUMBER, val.to_string())
    }

    fn neg(&self) -> Result<Value, Error> {
        let val: f64 = -self.parse_to_f64()?;
        Value::new(VT::NUMBER, val.to_string())
    }

    fn not(&self) -> Result<Value, Error> {
        let val: bool = !self.parse_to_bool()?;
        Value::new(VT::BOOLEAN, val.to_string())
    }

    fn add(&self, rval: Value) -> Result<Value, Error> {
        let val: String = match (&self.vtype, &rval.vtype) {
            (VT::STRING, _) |
            (_, VT::STRING) => self.value.clone() + &rval.value.clone(),
            _ => (self.parse_to_f64()? + rval.parse_to_f64()?).to_string()
        };
        Value::new(VT::NUMBER, val)
    }

    fn sub(&self, rval: Value) -> Result<Value, Error> {
        let val: f64 = self.parse_to_f64()? - rval.parse_to_f64()?;
        Value::new(VT::NUMBER, val.to_string())
    }

    fn mul(&self, rval: Value) -> Result<Value, Error> {
        let val: f64 = self.parse_to_f64()? * rval.parse_to_f64()?;
        Value::new(VT::NUMBER, val.to_string())
    }

    fn div(&self, rval: Value) -> Result<Value, Error> {
        let val: f64 = self.parse_to_f64()? / rval.parse_to_f64()?;
        Value::new(VT::NUMBER, val.to_string())
    }

    fn rem(&self, rval: Value) -> Result<Value, Error> {
        let val: f64 = self.parse_to_f64()? % rval.parse_to_f64()?;
        Value::new(VT::NUMBER, val.to_string())
    }

    fn eq(&self, rval: Value) -> Result<Value, Error> {
        let val: bool = self.value == rval.value;
        Value::new(VT::BOOLEAN, val.to_string())
    }

    fn ne(&self, rval: Value) -> Result<Value, Error> {
        let val: bool = self.value != rval.value;
        Value::new(VT::BOOLEAN, val.to_string())
    }

    fn lgr(&self, rval: Value) -> Result<Value, Error> {
        let val: bool = self.parse_to_f64()? > rval.parse_to_f64()?;
        Value::new(VT::BOOLEAN, val.to_string())
    }

    fn leq(&self, rval: Value) -> Result<Value, Error> {
        let val: bool = self.parse_to_f64()? >= rval.parse_to_f64()?;
        Value::new(VT::BOOLEAN, val.to_string())
    }

    fn smr(&self, rval: Value) -> Result<Value, Error> {
        let val: bool = self.parse_to_f64()? < rval.parse_to_f64()?;
        Value::new(VT::BOOLEAN, val.to_string())
    }

    fn seq(&self, rval: Value) -> Result<Value, Error> {
        let val: bool = self.parse_to_f64()? <= rval.parse_to_f64()?;
        Value::new(VT::BOOLEAN, val.to_string())
    }

    fn  and(&self, rval: Value) -> Result<Value, Error> {
        if self.parse_to_bool()? && rval.parse_to_bool()? {
            Ok(rval)
        } else {
            Value::new(VT::BOOLEAN, false.to_string())
        }
    }

    fn or(&self, rval: Value) -> Result<Value, Error> {
        if self.parse_to_bool()? {
            Ok(self.clone())
        } else if self.parse_to_bool()? {
            Ok(rval)
        } else {
            Value::new(VT::BOOLEAN, false.to_string())
        }
    }
}


impl Value {
    fn new(vtype: VT, value: String) -> Result<Self, Error> {
        Ok(Self { vtype, value })
    }
}


impl Error {
    fn cvt_to_vt_failed(name: &String) -> Result<Value, Error> {
        Err(Self {
            msg: format!("cannot convert `{}` to a value", name)
        })
    }

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

    fn cannot_assign(value: &String) -> Result<Value, Error> {
        Err(Self {
            msg: format!("cannot assign to token `{}`", value)
        })
    }
}
