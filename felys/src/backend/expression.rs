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
    pub(super) fn eval(
        &self,
        env: &mut Environ,
        out: &mut Output,
        ctr: &mut usize
    ) -> Result<Value, Error> {
        match self.ntype {
            NT::VALUE(_) => self.to_value(env),
            NT::BINOPTR => self.eval_binoptr(env, out, ctr),
            NT::UNAOPTR => self.eval_unaoptr(env, out, ctr),
            NT::CALLABLE => self.call(env, out, ctr),
        }
    }

    fn call(
        &self,
        env: &mut Environ,
        out: &mut Output,
        ctr: &mut usize
    ) -> Result<Value, Error> {
        let mut args: Vec<Value> = Vec::new();
        for param in self.nodes.iter() {
            let val: Value = param.eval(env, out, ctr)?;
            args.push(val);
        }
        env.call(&self.value, args, out, ctr)
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

    fn eval_binoptr(
        &self,
        env: &mut Environ,
        out: &mut Output,
        ctr: &mut usize
    ) -> Result<Value, Error> {
        let rval: Value = match self.nodes.last() {
            Some(node) => node.eval(env, out, ctr)?,
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
                node.eval(env, out, ctr)?
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

    fn eval_unaoptr(
        &self,
        env: &mut Environ,
        out: &mut Output,
        ctr: &mut usize
    ) -> Result<Value, Error> {
        let val: Value = match self.nodes.first() {
            Some(node) => node.eval(env, out, ctr)?,
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
        match (&self.vtype, &rval.vtype) {
            (VT::STRING, _) |
            (_, VT::STRING) => {
                let val: String = self.value.clone() + &rval.value.clone();
                Value::new(VT::STRING, val)
            },
            _ => {
                let val: String = (self.parse_to_f64()? + rval.parse_to_f64()?).to_string();
                Value::new(VT::NUMBER, val)
            }
        }
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

    fn and(&self, rval: Value) -> Result<Value, Error> {
        if self.parse_to_bool()? && rval.parse_to_bool()? {
            Ok(rval)
        } else {
            Value::new(VT::BOOLEAN, false.to_string())
        }
    }

    fn or(&self, rval: Value) -> Result<Value, Error> {
        if self.parse_to_bool()? {
            Ok(self.clone())
        } else if rval.parse_to_bool()? {
            Ok(rval)
        } else {
            Value::new(VT::BOOLEAN, false.to_string())
        }
    }
}





impl Error {
    fn cvt_to_vt_failed(name: &String) -> Result<Value, Error> {
        Err(Self {
            msg: format!("cannot convert `{}` to a value", name)
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
