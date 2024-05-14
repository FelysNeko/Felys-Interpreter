use crate::shared::{
    TokenType as TT,
    NodeType as NT,
    Error,
    Token,
    Node,
};
use super::Lexer;


impl Lexer<'_> {
    pub(super) fn parse_expression(&mut self) -> Result<Node, Error>{
        self._parse_assignment()
    }

    fn _parse_assignment(&mut self) -> Result<Node, Error> {
        let mut left: Node = self._parse_logical()?;

        while let Some(tk) = self.token.pop() {
            if tk.ttype == TT::NODE(NT::BINOPTR) && tk.value == "=" {
                let mut new: Node = Node::from(tk)?;
                let right: Node = self._parse_assignment()?;
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

    fn _parse_logical(&mut self) -> Result<Node, Error> {
        let mut left: Node = self._parse_compare()?;

        while let Some(tk) = self.token.pop() {
            if tk.ttype == TT::NODE(NT::BINOPTR) && (tk.value=="and" || tk.value=="or") {
                let mut new: Node = Node::from(tk)?;
                let right: Node = self._parse_compare()?;
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

    fn _parse_compare(&mut self) -> Result<Node, Error> {
        let mut left: Node = self._parse_additive()?;

        while let Some(tk) = self.token.pop() {
            if tk.ttype == TT::NODE(NT::BINOPTR) && (
                tk.value == "==" || tk.value == "!=" || tk.value == ">" ||
                tk.value == ">=" || tk.value == "<=" || tk.value == "<"
            ) {
                let mut new: Node = Node::from(tk)?;
                let right: Node = self._parse_additive()?;
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

    fn _parse_additive(&mut self) -> Result<Node, Error> {
        let mut left: Node = self._parse_multiply()?;

        while let Some(tk) = self.token.pop() {
            if tk.ttype == TT::NODE(NT::BINOPTR) && (tk.value == "+" || tk.value == "-") {
                let mut new: Node = Node::from(tk)?;
                let right: Node = self._parse_multiply()?;
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

    fn _parse_multiply(&mut self) -> Result<Node, Error> {
        let mut left: Node = self._parse_unary()?;

        while let Some(tk) = self.token.pop() {
            if tk.ttype == TT::NODE(NT::BINOPTR) && (
                tk.value == "*" || tk.value == "/" || tk.value == "%"
            ) {
                let mut new: Node = Node::from(tk)?;
                let right: Node = self._parse_unary()?;
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

    fn _parse_unary(&mut self) -> Result<Node, Error> {
        while let Some(tk) = self.token.pop() {
            if tk.ttype == TT::NODE(NT::UNAOPTR) {
                let mut new: Node = Node::from(tk)?;
                let node: Node = self._parse_unary()?;
                new.nodes.push(node);
                return Ok(new);
            } else {
                self.token.push(tk);
                break;
            }
        }
        self._parse_primary()
    }

    fn _parse_primary(&mut self) -> Result<Node, Error> {
        if let Some(tk) = self.token.last() {
            match tk.ttype {
                TT::NODE(NT::VALUE(_)) => self._parse_value(),
                TT::NODE(NT::CALLABLE) => self._parse_callable(),
                TT::LPAREN => self._parse_parentheses(),
                _ => Error::token_not_primary(&tk.value)
            }
        } else {
            Error::no_more_token()
        }
    }

    fn _parse_value(&mut self) -> Result<Node, Error> {
        if let Some(tk) = self.token.pop() {
            Node::from(tk)
        } else {
            Error::no_more_token()
        }
    }

    fn _parse_callable(&mut self) -> Result<Node, Error> {
        if let Some(tk) = self.token.pop() {
            let mut callable: Node = Node::from(tk)?;
            self.eat(TT::LPAREN)?;

            while let Some(tk) = self.token.last() {
                if tk.ttype == TT::COMMA {
                    self.eat(TT::COMMA)?;
                } else if tk.ttype == TT::RPAREN {
                    self.eat(TT::RPAREN)?;
                    return Ok(callable);
                } else {
                    let next: Node = self.parse_expression()?;
                    callable.nodes.push(next);
                }
            }
        }
        Error::no_more_token()
    }

    fn _parse_parentheses(&mut self) -> Result<Node, Error> {
        self.eat(TT::LPAREN)?;
        let expr: Node = self.parse_expression()?;
        self.eat(TT::RPAREN)?;
        Ok(expr)
    }
}

impl Node {
    pub(super) fn from(tk: Token) -> Result<Self, Error> {
        if let TT::NODE(ntype) = tk.ttype {
            Ok(Self { ntype, value: tk.value, nodes: Vec::new() })
        } else {
            Error::invalid_token_type(tk.ttype)
        }
    }
}


impl Error {
    fn invalid_token_type(t: TT) -> Result<Node, Error> {
        Err(Self { msg: format!("cannot convert token {:?} to node", t) })
    }

    fn token_not_primary(v: &String) -> Result<Node, Error> {
        Err(Self { msg: format!("expect primary token, but see `{}`", v) })
    }

    pub(super) fn no_more_token() -> Result<Node, Error> {
        Err(Self { msg: format!("no more token to parse") })
    }
}