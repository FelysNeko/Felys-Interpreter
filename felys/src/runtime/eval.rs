use super::Value;
use crate::frontend::Node;
use crate::frontend::TokenType as TT;
use std::ops;


impl Value {
    fn from(n:Node) -> Self {
        Value::new(n.kind, n.value)
    }

    fn null() -> Self {
        Self {
            kind: TT::Null,
            value: String::new()
        }
    }

    fn new(kind:TT, value:String) -> Self {
        if kind==TT::Integer || kind==TT::String {
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
            TT::Identifier |
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
            "+" => lval + rval,
            "-" => lval - rval,
            "*" => lval * rval,
            "/" => lval / rval,
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
            "+" => val,
            "-" => -val,
            "!" => !val,
            _ => Value::null()
        }
    }
}


impl ops::Add<Value> for Value {
    type Output = Value;
    fn add(self, _rhs:Value) -> Self::Output {
        return Value::null();
    }
}

impl ops::Sub<Value> for Value {
    type Output = Value;
    fn sub(self, _rhs:Value) -> Self::Output {
        return Value::null();
    }
}

impl ops::Mul<Value> for Value {
    type Output = Value;
    fn mul(self, _rhs:Value) -> Self::Output {
        return Value::null();
    }
}

impl ops::Div<Value> for Value {
    type Output = Value;
    fn div(self, _rhs:Value) -> Self::Output {
        return Value::null();
    }
}

impl ops::Neg for Value {
    type Output = Value;
    fn neg(self) -> Self::Output {
        return Value::null();
    }
}

impl ops::Not for Value {
    type Output = Value;
    fn not(self) -> Self::Output {
        return Value::null();
    }
}