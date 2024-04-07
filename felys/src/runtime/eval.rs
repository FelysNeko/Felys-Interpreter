use super::Value;
use super::RuntimeType as RT;
use crate::frontend::Node;
use crate::frontend::TokenType as TT;
use std::process::exit;


impl Value {
    fn from(n:Node) -> Self {
        match n.kind {
            TT::Integer => Self { kind: RT::Integer, value: n.value },
            TT::String => Self { kind: RT::String, value: n.value },
            _ => Value::null()
        }
    }

    fn null() -> Self {
        Self {
            kind: RT::Null,
            value: String::new()
        }
    }

    fn new(kind:RT, value:String) -> Self {
        if kind==RT::Integer || kind==RT::String || kind==RT::Bool {
            Self { kind, value }
        } else {
            Value::null()
        }
    }
}


impl Node {
    pub fn eval(self) -> Value {
        match self.kind {
            TT::BinaryOperator => self.eval_binary_operation(),
            TT::UnaryOperator => self.eval_unary_operation(),
            TT::Identifier => self.eval_identifier(),
            TT::Integer |
            TT::String => Value::from(self),
            _ => Value::null()
        }
    }

    fn eval_binary_operation(mut self) -> Value {
        let rval: Value = match self.branch.pop() {
            Some(v) => v.eval(),
            None => Value::null()
        };
        
        let lval: Value = match self.branch.pop() {
            Some(v) => v.eval(),
            None => Value::null()
        };

        match self.value.as_str() {
            "+" => lval.add(rval),
            "-" => lval.sub(rval),
            "*" => lval.mul(rval),
            "/" => lval.div(rval),
            "=" => rval,
            _ => Value::null()
        }
    }

    fn eval_unary_operation(mut self) -> Value {
        let val: Value = match self.branch.pop() {
            Some(v) => v.eval(),
            None => Value::null()
        };
    
        match self.value.as_str() {
            "+" => val.pos(),
            "-" => val.neg(),
            "!" => val.not(),
            _ => Value::null()
        }
    }

    fn eval_identifier(self) -> Value {
        Value::null()
    }
}


impl Value {
    fn integer_to_isize(&self) -> isize {
        match self.value.parse::<isize>() {
            Ok(num) => num,
            Err(_) => exit(1)
        }
    }

    fn add(self, rval:Value) -> Self {
        if self.kind==RT::Integer && rval.kind==RT::Integer {
            let val: isize = self.integer_to_isize() + rval.integer_to_isize();
            Value::new(RT::Integer, val.to_string())
        } else if self.kind==RT::String && rval.kind==RT::String {
            let val: String = self.value + &rval.value;
            Value::new(RT::String, val)
        } else {
            Value::null()
        }
    }

    fn sub(self, rval:Value) -> Self {
        if self.kind==RT::Integer && rval.kind==RT::Integer {
            let val: isize = self.integer_to_isize() - rval.integer_to_isize();
            Value::new(RT::Integer, val.to_string())
        } else {
            Value::null()
        }
    }

    fn mul(self, rval:Value) -> Self {
        if self.kind==RT::Integer && rval.kind==RT::Integer {
            let val: isize = self.integer_to_isize() * rval.integer_to_isize();
            Value::new(RT::Integer, val.to_string())
        } else {
            Value::null()
        }
    }

    fn div(self, rval:Value) -> Self {
        if self.kind==RT::Integer && rval.kind==RT::Integer {
            let val: isize = self.integer_to_isize() / rval.integer_to_isize();
            Value::new(RT::Integer, val.to_string())
        } else {
            Value::null()
        }
    }

    fn pos(self) -> Self {
        if self.kind == RT::Integer {
            self
        } else {
            Value::null()
        }
    }

    fn neg(mut self) -> Self {
        if self.kind == RT::Integer {
            self.value.insert(0, '-');
            Value::new(RT::Integer, self.value)
        } else {
            Value::null()
        }
    }

    fn not(self) -> Self {
        if self.kind == RT::Integer {
            match self.integer_to_isize() {
                0 => Value::new(RT::Bool, "true".to_string()),
                _ => Value::new(RT::Bool, "false".to_string())
            }
        } else {
            Value::null()
        }
    }
}