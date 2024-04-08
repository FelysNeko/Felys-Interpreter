use super::Value;
use super::RuntimeType as RT;
use super::Scope;
use crate::frontend::Node;
use crate::frontend::TokenType as TT;
use std::process::exit;


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
            make_bool!(true)
        } else {
            make_bool!(false)
        }
    };
}

macro_rules! return_value {
    ($k:expr, $e:expr) => {
        Value::new($k, $e.to_string())
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
    pub fn eval(self, env:&mut Scope) -> Value {
        match self.kind {
            TT::BinaryOperator => self.eval_binary_operation(env),
            TT::UnaryOperator => self.eval_unary_operation(env),
            TT::Identifier => env.get(self.value),
            TT::Integer |
            TT::String => Value::from(self),
            _ => Value::null()
        }
    }

    fn eval_binary_operation(mut self, env:&mut Scope) -> Value {
        let rval: Value = match self.branch.pop() {
            Some(v) => v.eval(env),
            None => Value::null()
        };

        let lval: Value = match self.branch.pop() {
            Some(v) => if self.value.as_str() == "=" {
                env.assign(v.value, rval.clone());
                return rval;
            } else {
                v.eval(env)
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
            _ => Value::null()
        }
    }

    fn eval_unary_operation(mut self, env:&mut Scope) -> Value {
        let val: Value = match self.branch.pop() {
            Some(v) => v.eval(env),
            None => Value::null()
        };
    
        match self.value.as_str() {
            "+" => val.pos(),
            "-" => val.neg(),
            "!" => val.not(),
            _ => Value::null()
        }
    }
}


impl Value {
    fn _parse(&self) -> isize {
        if self.kind == RT::Integer {
            match self.value.parse::<isize>() {
                Ok(num) => num,
                Err(_) => exit(1)
            }
        } else if self.kind == RT::Bool {
            if self.value.as_str() == "true" { 1 } else { 0 }
        } else {
            exit(1)
        }
    }

    fn _bool(&self) -> bool {
        match self.kind {
            RT::Integer |
            RT::Bool => self._parse() != 0,
            RT::String => self.value.len() != 0,
            RT::Null => false
        }
    }

    fn add(self, rval:Value) -> Self {
        if self.kind==RT::String && rval.kind==RT::String {
            return_value!(RT::String, self.value + &rval.value)
        } else {
            return_value!(RT::Integer, self._parse() + rval._parse())
        }
    }

    fn sub(self, rval:Value) -> Self {
        return_value!(RT::Integer, self._parse() - rval._parse())
    }

    fn mul(self, rval:Value) -> Self {
        return_value!(RT::Integer, self._parse() * rval._parse())
    }

    fn div(self, rval:Value) -> Self {
        return_value!(RT::Integer, self._parse() / rval._parse())
    }

    fn rem(self, rval:Value) -> Self {
        return_value!(RT::Integer, self._parse() % rval._parse())
    }

    fn gt(self, rval:Value) -> Self {
        return_bool!(self._parse() > rval._parse())
    }

    fn ge(self, rval:Value) -> Self {
        return_bool!(self._parse() >= rval._parse())
    }

    fn ls(self, rval:Value) -> Self {
        return_bool!(self._parse() < rval._parse())
    }

    fn le(self, rval:Value) -> Self {
        return_bool!(self._parse() <= rval._parse())
    }

    fn eq(self, rval:Value) -> Self {
        return_bool!(self.value == rval.value)
    }

    fn nq(self, rval:Value) -> Self {
        return_bool!(self.value != rval.value)
    }

    fn and(self, rval:Value) -> Self {
        if self._bool() == false || rval._bool() == false {
            make_bool!(false)
        } else {
            rval
        }
    }

    fn or(self, rval:Value) -> Self {
        if self._bool() == true {
            return self;
        }
        if rval._bool() == true {
            return rval;
        }
        make_bool!(false)
    }

    fn pos(self) -> Self {
        return_value!(RT::Integer, self._parse())
    }

    fn neg(self) -> Self {
        return_value!(RT::Integer, -self._parse())
    }

    fn not(self) -> Self {
        return_bool!(self._parse() == 0)
    }
}

