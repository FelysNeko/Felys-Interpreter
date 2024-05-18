use crate::shared::{
    TokenType as TT,
    NodeType as NT,
    Error,
    Node,
};
use super::Lexer;


impl Lexer<'_> {
    pub(super) fn parse_expression(&mut self) -> Result<Node, Error>{
        self.parse_assignment()
    }

    fn parse_assignment(&mut self) -> Result<Node, Error> {
        let mut left: Node = self.parse_logical()?;

        while let Some(tk) = self.token.pop() {
            if tk.ttype == TT::NODE(NT::BINOPTR) && tk.value == "=" {
                let mut new: Node = Node::from(tk)?;
                let right: Node = self.parse_assignment()?;
                new.nodes.push(left);
                new.nodes.push(right);
                left = new;
            } else {
                self.token.push(tk);
                break;
            }
        }
        Ok(left)
    }

    fn parse_logical(&mut self) -> Result<Node, Error> {
        let mut left: Node = self.parse_compare()?;

        while let Some(tk) = self.token.pop() {
            if tk.ttype == TT::NODE(NT::BINOPTR) && (tk.value=="and" || tk.value=="or") {
                let mut new: Node = Node::from(tk)?;
                let right: Node = self.parse_compare()?;
                new.nodes.push(left);
                new.nodes.push(right);
                left = new;
            } else {
                self.token.push(tk);
                break;
            }
        }
        Ok(left)
    }

    fn parse_compare(&mut self) -> Result<Node, Error> {
        let mut left: Node = self.parse_additive()?;

        while let Some(tk) = self.token.pop() {
            if tk.ttype == TT::NODE(NT::BINOPTR) && (
                tk.value == "==" || tk.value == "!=" || tk.value == ">" ||
                tk.value == ">=" || tk.value == "<=" || tk.value == "<"
            ) {
                let mut new: Node = Node::from(tk)?;
                let right: Node = self.parse_additive()?;
                new.nodes.push(left);
                new.nodes.push(right);
                left = new;
            } else {
                self.token.push(tk);
                break;
            }
        }
        Ok(left)
    }

    fn parse_additive(&mut self) -> Result<Node, Error> {
        let mut left: Node = self.parse_multiply()?;

        while let Some(tk) = self.token.pop() {
            if tk.ttype == TT::NODE(NT::BINOPTR) && (tk.value == "+" || tk.value == "-") {
                let mut new: Node = Node::from(tk)?;
                let right: Node = self.parse_multiply()?;
                new.nodes.push(left);
                new.nodes.push(right);
                left = new;
            } else {
                self.token.push(tk);
                break;
            }
        }
        Ok(left)
    }

    fn parse_multiply(&mut self) -> Result<Node, Error> {
        let mut left: Node = self.parse_unary()?;

        while let Some(tk) = self.token.pop() {
            if tk.ttype == TT::NODE(NT::BINOPTR) && (
                tk.value == "*" || tk.value == "/" || tk.value == "%"
            ) {
                let mut new: Node = Node::from(tk)?;
                let right: Node = self.parse_unary()?;
                new.nodes.push(left);
                new.nodes.push(right);
                left = new;
            } else {
                self.token.push(tk);
                break;
            }
        }
        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Node, Error> {
        while let Some(tk) = self.token.pop() {
            if tk.ttype == TT::NODE(NT::UNAOPTR) {
                let mut new: Node = Node::from(tk)?;
                let node: Node = self.parse_unary()?;
                new.nodes.push(node);
                return Ok(new);
            } else {
                self.token.push(tk);
                break;
            }
        }
        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Result<Node, Error> {
        if let Some(tk) = self.token.last() {
            match tk.ttype {
                TT::NODE(NT::VALUE(_)) => self.parse_value(),
                TT::NODE(NT::CALLABLE) => self.parse_callable(),
                TT::LPAREN => self.parse_parentheses(),
                _ => Error::token_not_primary(&tk.value)
            }
        } else {
            Error::node_no_more_token()
        }
    }

    fn parse_value(&mut self) -> Result<Node, Error> {
        if let Some(tk) = self.token.pop() {
            Node::from(tk)
        } else {
            Error::node_no_more_token()
        }
    }

    fn parse_callable(&mut self) -> Result<Node, Error> {
        if let Some(tk) = self.token.pop() {
            let mut callable: Node = Node::from(tk)?;
            self.eat(TT::LPAREN)?;

            loop {
                let next: Node = self.parse_expression()?;
                callable.nodes.push(next);
    
                if let Some(s) = self.token.pop() {
                    match s.ttype {
                        TT::COMMA => (),
                        TT::RPAREN => return Ok(callable),
                        _ => return Error::invalid_param(s.value)
                    }
                }
            };
        }
        Error::node_no_more_token()
    }

    fn parse_parentheses(&mut self) -> Result<Node, Error> {
        self.eat(TT::LPAREN)?;
        let expr: Node = self.parse_expression()?;
        self.eat(TT::RPAREN)?;
        Ok(expr)
    }
}


impl Error {
    fn invalid_param(s: String) -> Result<Node, Error> {
        Err(Self { msg: format!("expect `)` or `,`, but see `{}`", s)})
    }

    fn token_not_primary(v: &String) -> Result<Node, Error> {
        Err(Self { msg: format!("expect primary token, but see `{}`", v) })
    }

    pub(super) fn node_no_more_token() -> Result<Node, Error> {
        Err(Self { msg: format!("no more token to parse") })
    }
}