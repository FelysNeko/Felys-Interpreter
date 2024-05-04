use std::process::exit;

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
    pub fn eval(&mut self, env: &mut Scope) -> Value {
        match self.kind {
            TT::STRING |
            TT::NUMBER |
            TT::BOOLEAN => Value::from(self),
            TT::IDENT => env.get(&self.value),
            TT::BINOPTR => self._eval_binoptr(env),
            TT::UNAOPTR => self._eval_unaoptr(env),
            _ => {
                println!("cannot eval [{:?}] operation for now", self.kind);
                exit(1);
            }
        }
    }

    fn _eval_unaoptr(&mut self, env: &mut Scope) -> Value {
        let val: Value = match self.branch.first_mut() {
            Some(node) => node.eval(env),
            None => exit(1)
        };

        match self.value.as_str() {
            "+" => val._pos(),
            "-" => val._neg(),
            "!" => val._not(),
            _ => {
                println!("operator [{}] not impl", self.value);
                exit(1)
            }
        }
    }

    fn _eval_binoptr(&mut self, env: &mut Scope) -> Value {
        let rval: Value = match self.branch.last_mut() {
            Some(node) => node.eval(env),
            None => {
                println!("right value not shown up");
                exit(1);
            }
        };

        let lval: Value = match self.branch.first_mut() {
            Some(node) => if self.value.as_str() == "=" {
                if node.kind == TT::IDENT {
                    env.set(&node.value, &rval);
                    return rval;
                } else {
                    println!("cannot assign to [{:?}]", node.kind);
                    exit(1)
                }
            } else {
                node.eval(env)
            },
            None => {
                println!("left value not shown up");
                exit(1)
            }
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
            _ => {
                println!("operator [{}] not impl", self.value);
                exit(1)
            }
        }
    }
}


impl Value {
    fn _parse_to_f64(&self) -> f64 {
        if self.kind == RT::NUMBER {
            match self.value.parse::<f64>() {
                Ok(num) => num,
                Err(_) => {
                    println!("cannot parse number [{}]", self.value);
                    exit(1);
                }
            }
        } else {
            println!("cannot parse number [{}]", self.value);
            exit(1)
        }
    }

    pub fn _parse_to_bool(&self) -> bool {
        match self.kind {
            RT::BOOLEAN => if self.value.as_str() == "true" { true } else { false },
            RT::STRING => self.value.len() != 0,
            RT::NUMBER => self._parse_to_f64() != 0.0,
            _ => {
                println!("cannot parse bool [{}]", self.value);
                exit(1)
            }
        }
    }

    fn _add(&self, rval: Value) -> Value {
        let val: f64 = self._parse_to_f64() + rval._parse_to_f64();
        Value::new(RT::NUMBER, val.to_string())
    }

    fn _sub(&self, rval: Value) -> Value {
        let val: f64 = self._parse_to_f64() - rval._parse_to_f64();
        Value::new(RT::NUMBER, val.to_string())
    }

    fn _mul(&self, rval: Value) -> Value {
        let val: f64 = self._parse_to_f64() * rval._parse_to_f64();
        Value::new(RT::NUMBER, val.to_string())
    }

    fn _div(&self, rval: Value) -> Value {
        let val: f64 = self._parse_to_f64() / rval._parse_to_f64();
        Value::new(RT::NUMBER, val.to_string())
    }

    fn _mod(&self, rval: Value) -> Value {
        let val: f64 = self._parse_to_f64() % rval._parse_to_f64();
        Value::new(RT::NUMBER, val.to_string())
    }

    fn _eq(&self, rval: Value) -> Value {
        let val: bool = self.value == rval.value;
        Value::new(RT::BOOLEAN, val.to_string())
    }

    fn _ne(&self, rval: Value) -> Value {
        let val: bool = self.value != rval.value;
        Value::new(RT::BOOLEAN, val.to_string())
    }

    fn _lgr(&self, rval: Value) -> Value {
        let val: bool = self._parse_to_f64() > rval._parse_to_f64();
        Value::new(RT::BOOLEAN, val.to_string())
    }

    fn _leq(&self, rval: Value) -> Value {
        let val: bool = self._parse_to_f64() >= rval._parse_to_f64();
        Value::new(RT::BOOLEAN, val.to_string())
    }

    fn _smr(&self, rval: Value) -> Value {
        let val: bool = self._parse_to_f64() < rval._parse_to_f64();
        Value::new(RT::BOOLEAN, val.to_string())
    }

    fn _seq(&self, rval: Value) -> Value {
        let val: bool = self._parse_to_f64() <= rval._parse_to_f64();
        Value::new(RT::BOOLEAN, val.to_string())
    }

    fn  _and(&self, rval: Value) -> Value {
        if self._parse_to_bool() && rval._parse_to_bool() {
            rval
        } else {
            Value::new(RT::BOOLEAN, false.to_string())
        }
    }

    fn _or(&self, rval: Value) -> Value {
        if self._parse_to_bool() {
            self.clone()
        } else if self._parse_to_bool() {
            rval
        } else {
            Value::new(RT::BOOLEAN, false.to_string())
        }
    }

    fn _pos(&self) -> Value {
        let val: f64 = self._parse_to_f64();
        Value::new(RT::NUMBER, val.to_string())
    }

    fn _neg(&self) -> Value {
        let val: f64 = -self._parse_to_f64();
        Value::new(RT::NUMBER, val.to_string())
    }

    fn _not(&self) -> Value {
        let val: bool = !self._parse_to_bool();
        Value::new(RT::BOOLEAN, val.to_string())
    }
}