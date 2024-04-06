use crate::frontend::Node;
use crate::frontend::TokenType as TT;
use std::ops;

impl Node {
    pub fn eval(&mut self) -> Node {
        match self.kind {
            TT::BinaryOperator => self.binary_operation(),
            TT::UnaryOperator => self.unary_operation(),
            _ => Node::null()
        }
    }

    fn binary_operation(&mut self) -> Self {
        let rhs: Node = self.branch.pop().unwrap_or(Node::null());
        let lhs: Node = self.branch.pop().unwrap_or(Node::null());
        match self.value.as_str() {
            "+" => rhs + lhs,
            "-" => rhs - lhs,
            "*" => rhs * lhs,
            "/" => rhs / lhs,
            "=" => rhs,
            _ => Node::null()
        }
    }

    fn unary_operation(&mut self) -> Self {
        let node: Node = self.branch.pop().unwrap_or(Node::null());
        match self.value.as_str() {
            "+" => node,
            "-" => -node,
            "!" => !node,
            _ => Node::null()
        }
    }
}


impl ops::Add<Node> for Node {
    type Output = Node;
    fn add(self, _rhs: Node) -> Node {
        Node::null()
    }
}

impl ops::Sub<Node> for Node {
    type Output = Node;
    fn sub(self, _rhs: Node) -> Node {
        Node::null()
    }
}

impl ops::Mul<Node> for Node {
    type Output = Node;
    fn mul(self, _rhs: Node) -> Node {
        Node::null()
    }
}

impl ops::Div<Node> for Node {
    type Output = Node;
    fn div(self, _rhs: Node) -> Node {
        Node::null()
    }
}

impl ops::Neg for Node {
    type Output = Node;
    fn neg(self) -> Node {
        Node::null()
    }
}

impl ops::Not for Node {
    type Output = Node;
    fn not(self) -> Node {
        Node::null()
    }
}