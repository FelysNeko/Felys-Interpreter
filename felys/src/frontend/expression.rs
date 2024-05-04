use crate::core::Error;
use crate::core::frontend::{
    TokenType as TT,
    Lexer,
    Token,
    Node
};


impl Lexer<'_> {
    pub fn _parse_expression(&mut self) -> Result<Node, Error> {
        self._parse_assignment()
    }

    fn _parse_assignment(&mut self) -> Result<Node, Error> {
        let mut left: Node = self._parse_logical()?;

        while let Some(tk) = self.tokens.pop() {
            if tk.kind == TT::BINOPTR && tk.value == "=" {
                let mut new: Node = Node::from(tk);
                let right: Node = self._parse_assignment()?;
                new.push(left);
                new.push(right);
                left = new;
            } else {
                self.tokens.push(tk);
                break;
            }
        }
        Ok(left)
    }

    fn _parse_logical(&mut self) -> Result<Node, Error> {
        let mut left: Node = self._parse_compare()?;

        while let Some(tk) = self.tokens.pop() {
            if tk.kind == TT::BINOPTR && (tk.value=="and" || tk.value=="or") {
                let mut new: Node = Node::from(tk);
                let right: Node = self._parse_compare()?;
                new.push(left);
                new.push(right);
                left = new;
            } else {
                self.tokens.push(tk);
                break;
            }
        }
        Ok(left)
    }

    fn _parse_compare(&mut self) -> Result<Node, Error> {
        let mut left: Node = self._parse_additive()?;

        while let Some(tk) = self.tokens.pop() {
            if tk.kind == TT::BINOPTR && (
                tk.value == "==" || tk.value == "!=" || tk.value == ">" ||
                tk.value == ">=" || tk.value == "<=" || tk.value == "<"
            ) {
                let mut new: Node = Node::from(tk);
                let right: Node = self._parse_additive()?;
                new.push(left);
                new.push(right);
                left = new;
            } else {
                self.tokens.push(tk);
                break;
            }
        }
        Ok(left)
    }

    fn _parse_additive(&mut self) -> Result<Node, Error> {
        let mut left: Node = self._parse_multiply()?;

        while let Some(tk) = self.tokens.pop() {
            if tk.kind == TT::BINOPTR && (tk.value == "+" || tk.value == "-") {
                let mut new: Node = Node::from(tk);
                let right: Node = self._parse_multiply()?;
                new.push(left);
                new.push(right);
                left = new;
            } else {
                self.tokens.push(tk);
                break;
            }
        }
        Ok(left)
    }

    fn _parse_multiply(&mut self) -> Result<Node, Error> {
        let mut left: Node = self._parse_unary()?;

        while let Some(tk) = self.tokens.pop() {
            if tk.kind == TT::BINOPTR && (
                tk.value == "*" || tk.value == "/" || tk.value == "%"
            ) {
                let mut new: Node = Node::from(tk);
                let right: Node = self._parse_unary()?;
                new.push(left);
                new.push(right);
                left = new;
            } else {
                self.tokens.push(tk);
                break;
            }
        }
        Ok(left)
    }

    fn _parse_unary(&mut self) -> Result<Node, Error> {
        while let Some(tk) = self.tokens.pop() {
            if tk.kind == TT::UNAOPTR {
                let mut new: Node = Node::from(tk);
                let node: Node = self._parse_unary()?;
                new.push(node);
                return Ok(new);
            } else {
                self.tokens.push(tk);
                break;
            }
        }
        self._parse_primary()
    }

    fn _parse_primary(&mut self) -> Result<Node, Error> {
        if let Some(tk) = self.tokens.pop() {
            match tk.kind {
                TT::IDENT |
                TT::NUMBER |
                TT::BOOLEAN |
                TT::STRING => Ok(Node::from(tk)),
                TT::LPAREN => self._parse_parentheses(),
                _ => Error::next_token_not_primary(&tk.value)
            }
        } else {
            Error::no_more_token_vague()
        }
    }

    fn _parse_parentheses(&mut self) -> Result<Node, Error> {
        let expr: Node = self._parse_expression()?;
        self._must_eat(TT::RPAREN)?;
        Ok(expr)
    }

    pub fn _must_eat(&mut self, kind: TT) -> Result<(), Error>{
        if let Some(cp) = self.tokens.pop() {
            if cp.kind != kind {
                Error::incorrect_next_token(kind, cp.kind)
            } else {
                Ok(())
            }
        } else {
            Error::no_more_token(kind)
        }
    }
}


impl Node {
    pub fn from(tk: Token) -> Self {
        Self {
            kind: tk.kind,
            value: tk.value,
            branch: Vec::new()
        }
    }

    pub fn null() -> Self {
        Node::from(Token::null())
    }

    pub fn push(&mut self, n:Node) {
        self.branch.push(n)
    }
}


impl Error {
    fn incorrect_next_token(expect: TT, this: TT) -> Result<(), Error> {
        Err(Self {
            msg: format!("expect [{:?}], but see [{:?}]", expect, this)
        })
    }

    fn next_token_not_primary(value: &String) -> Result<Node, Error> {
        Err(Self {
            msg: format!("expect primary token, but see [{:?}]", value)
        })
    }

    fn no_more_token(expect: TT) -> Result<(), Error> {
        Err(Self {
            msg: format!("expect [{:?}], but no more", expect)
        })
    }

    fn no_more_token_vague() -> Result<Node, Error> {
        Err(Self {
            msg: format!("expect primary token, but no more")
        })
    }
}