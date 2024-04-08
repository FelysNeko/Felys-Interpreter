use super::Value;
use super::RuntimeType as RT;
use super::Scope;
use crate::error::Handler;
use crate::error::Handler::{Good, Bad};
use crate::error::Msg;
use crate::frontend::Node;
use crate::frontend::TokenType as TT;
use crate::return_if_err;


macro_rules! make_bool {
    ($b:tt) => {
        if $b == true {
            Value::new(RT::Bool, String::from("true"))
        } else {
            Value::new(RT::Bool, String::from("false"))
        }
    };
}

macro_rules! return_bool {
    ($e:expr) => {
        if $e {
            Good(make_bool!(true))
        } else {
            Good(make_bool!(false))
        }
    };
}

macro_rules! return_value {
    ($k:expr, $e:expr) => {
        Good(Value::new($k, $e.to_string()))
    };
}


impl Value {
    fn from(n:Node) -> Self {
        match n.kind {
            TT::Integer => Self { kind: RT::Integer, value: n.value },
            TT::String => Self { kind: RT::String, value: n.value },
            _ => Value::null()
        }
    }

    pub fn null() -> Self {
        Self {
            kind: RT::Null,
            value: String::new()
        }
    }

    pub fn new(kind:RT, value:String) -> Self {
        if kind==RT::Integer || kind==RT::String || kind==RT::Bool {
            Self { kind, value }
        } else {
            Value::null()
        }
    }
}


impl Node {
    pub fn eval(self, env:&mut Scope) -> Handler<Value, String> {
        match self.kind {
            TT::BinaryOperator => self.eval_binary_operation(env),
            TT::UnaryOperator => self.eval_unary_operation(env),
            TT::Identifier => env.get(self.value),
            TT::Integer |
            TT::String => Good(Value::from(self)),
            _ => Bad(Msg::unknown())
        }
    }

    fn eval_binary_operation(mut self, env:&mut Scope) -> Handler<Value, String> {
        let rval: Value = match self.branch.pop() {
            Some(v) => return_if_err!(v.eval(env)),
            None => Value::null()
        };

        let lval: Value = match self.branch.pop() {
            Some(v) => if self.value.as_str() == "=" {
                env.assign(v.value, rval.clone());
                return Good(rval);
            } else {
                return_if_err!(v.eval(env))
            },
            None => Value::null()
        };

        match self.value.as_str() {
            "+" => lval.add(rval),
            "-" => lval.sub(rval),
            "*" => lval.mul(rval),
            "/" => lval.div(rval),
            "%" => lval.rem(rval),
            ">" => lval.gt(rval),
            "<" => lval.ls(rval),
            ">=" => lval.ge(rval),
            "<=" => lval.le(rval),
            "==" => lval.eq(rval),
            "!=" => lval.nq(rval),
            "&&" => lval.and(rval),
            "||" => lval.or(rval),
            _ => Bad(Msg::binary_operation_not_implented(&self))
        }
    }

    fn eval_unary_operation(mut self, env:&mut Scope) -> Handler<Value, String> {
        let val: Value = match self.branch.pop() {
            Some(v) => return_if_err!(v.eval(env)),
            None => Value::null()
        };
    
        match self.value.as_str() {
            "+" => val.pos(),
            "-" => val.neg(),
            "!" => val.not(),
            _ => Bad(Msg::unary_operation_not_implented(&self))
        }
    }
}


impl Value {
    fn _parse(&self) -> Handler<isize, String> {
        if self.kind == RT::Integer {
            match self.value.parse::<isize>() {
                Ok(num) => Good(num),
                Err(_) => Bad(Msg::cannot_parse_to_isize(self))
            }
        } else if self.kind == RT::Bool {
            if self.value.as_str() == "true" {
                Good(1)
            } else {
                Good(0)
            }
        } else {
            Bad(Msg::cannot_parse_to_isize(self))
        }
    }

    fn _bool(&self) -> Handler<bool, String> {
        match self.kind {
            RT::Integer |
            RT::Bool => Good(return_if_err!(self._parse()) != 0),
            RT::String => Good(self.value.len() != 0),
            RT::Null => Good(false)
        }
    }

    fn add(self, rval:Value) -> Handler<Self, String> {
        if self.kind==RT::String && rval.kind==RT::String {
            return_value!(RT::String, self.value + &rval.value)
        } else {
            return_value!(
                RT::Integer, 
                return_if_err!(self._parse()) + return_if_err!(rval._parse())
            )
        }
    }

    fn sub(self, rval:Value) -> Handler<Self, String> {
        return_value!(
            RT::Integer,
            return_if_err!(self._parse()) - return_if_err!(rval._parse())
        )
    }

    fn mul(self, rval:Value) -> Handler<Self, String> {
        return_value!(
            RT::Integer,
            return_if_err!(self._parse()) * return_if_err!(rval._parse())
        )
    }

    fn div(self, rval:Value) -> Handler<Self, String> {
        return_value!(
            RT::Integer,
            return_if_err!(self._parse()) / return_if_err!(rval._parse())
        )
    }

    fn rem(self, rval:Value) -> Handler<Self, String> {
        return_value!(
            RT::Integer,
            return_if_err!(self._parse()) % return_if_err!(rval._parse())
        )
    }

    fn gt(self, rval:Value) -> Handler<Self, String> {
        return_bool!(
            return_if_err!(self._parse()) > return_if_err!(rval._parse())
        )
    }

    fn ge(self, rval:Value) -> Handler<Self, String> {
        return_bool!(
            return_if_err!(self._parse()) >= return_if_err!(rval._parse())
        )
    }

    fn ls(self, rval:Value) -> Handler<Self, String> {
        return_bool!(
            return_if_err!(self._parse()) < return_if_err!(rval._parse())
        )
    }

    fn le(self, rval:Value) -> Handler<Self, String> {
        return_bool!(
            return_if_err!(self._parse()) <= return_if_err!(rval._parse())
        )
    }

    fn eq(self, rval:Value) -> Handler<Self, String> {
        return_bool!(
            return_if_err!(self._parse()) == return_if_err!(rval._parse())
        )
    }

    fn nq(self, rval:Value) -> Handler<Self, String> {
        return_bool!(
            return_if_err!(self._parse()) != return_if_err!(rval._parse())
        )
    }

    fn and(self, rval:Value) -> Handler<Self, String> {
        if return_if_err!(self._bool()) == false || return_if_err!(rval._bool()) == false {
            Good(make_bool!(false))
        } else {
            Good(rval)
        }
    }

    fn or(self, rval:Value) -> Handler<Self, String> {
        if return_if_err!(self._bool()) == true {
            return Good(self);
        }
        if return_if_err!(rval._bool()) == true {
            return Good(rval);
        }
        Good(make_bool!(false))
    }

    fn pos(self) -> Handler<Self, String> {
        return_value!(
            RT::Integer,
            return_if_err!(self._parse())
        )
    }

    fn neg(self) -> Handler<Self, String> {
        return_value!(
            RT::Integer,
            -return_if_err!(self._parse())
        )
    }

    fn not(self) -> Handler<Self, String> {
        return_bool!(
            return_if_err!(self._parse()) == 0
        )
    }
}

