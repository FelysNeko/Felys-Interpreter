use crate::shared::{BinaryNode, BT, Error, Node, Program, ST, TT, UnaryNode};

impl Program {
    pub(super) fn parse_expression(&mut self) -> Result<Node, Error> {
        self.parse_assignement()
    }

    fn parse_assignement(&mut self) -> Result<Node, Error> {
        let mut left = self.parse_logical()?;

        while let Some(token) = self.tokens.pop() {
            if let TT::Bin(
                BT::Asn | BT::Ade | BT::Sue | BT::Mue | BT::Die | BT::Moe
            ) = token.kind {
                let right = self.parse_logical()?;
                left = BinaryNode::build(token.kind, left, right)?;
            } else {
                self.tokens.push(token);
                break;
            }
        }

        Ok(left)
    }

    fn parse_logical(&mut self) -> Result<Node, Error> {
        let mut left = self.parse_compare()?;

        while let Some(token) = self.tokens.pop() {
            if let TT::Bin(
                BT::And | BT::Xor | BT::Or
            ) = token.kind {
                let right = self.parse_compare()?;
                left = BinaryNode::build(token.kind, left, right)?;
            } else {
                self.tokens.push(token);
                break;
            }
        }

        Ok(left)
    }

    fn parse_compare(&mut self) -> Result<Node, Error> {
        let mut left = self.parse_additive()?;

        while let Some(token) = self.tokens.pop() {
            if let TT::Bin(
                BT::Gt | BT::Ge | BT::Lt | BT::Le | BT::Eq | BT::Ne
            ) = token.kind {
                let right = self.parse_additive()?;
                left = BinaryNode::build(token.kind, left, right)?;
            } else {
                self.tokens.push(token);
                break;
            }
        }

        Ok(left)
    }

    fn parse_additive(&mut self) -> Result<Node, Error> {
        let mut left = self.parse_multiply()?;

        while let Some(token) = self.tokens.pop() {
            if let TT::Bin(
                BT::Add | BT::Sub
            ) = token.kind {
                let right = self.parse_multiply()?;
                left = BinaryNode::build(token.kind, left, right)?;
            } else {
                self.tokens.push(token);
                break;
            }
        }

        Ok(left)
    }

    fn parse_multiply(&mut self) -> Result<Node, Error> {
        let mut left = self.parse_unary()?;

        while let Some(token) = self.tokens.pop() {
            if let TT::Bin(
                BT::Mod | BT::Mul | BT::Div
            ) = token.kind {
                let right = self.parse_unary()?;
                left = BinaryNode::build(token.kind, left, right)?;
            } else {
                self.tokens.push(token);
                break;
            }
        }
        
        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Node, Error> {
        if let Some(token) = self.tokens.pop() {
            if let TT::Una(_) = token.kind {
                let next = self.parse_unary()?;
                let temp = UnaryNode::build(token.kind, next)?;
                return Ok(temp)
            } else {
                self.tokens.push(token);
            }
        }
        
        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Result<Node, Error> {
        if let Some(token) = self.tokens.last() {
            match token.kind {
                TT::Val(_) => self.parse_literal(),
                TT::Identifier => self.parse_identifier(),
                TT::Sym(ST::LParen) => self.parse_lparen(),
                TT::Sym(ST::Pipe) => self.parse_function(),
                _ => Error::token_not_primary(&token.value)
            }
        } else {
            Error::no_more_token()
        }
    }

    fn parse_literal(&mut self) -> Result<Node, Error> {
        todo!()
    }
    
    fn parse_identifier(&mut self) -> Result<Node, Error> {
        todo!()
    }

    fn parse_function(&mut self) -> Result<Node, Error> {
        todo!()
    }

    fn parse_lparen(&mut self) -> Result<Node, Error> {
        todo!()
    }
}

impl Error {
    fn token_not_primary(s: &String) -> Result<Node, Error> {
        Err(Self { body: format!("token `{}` is not primary", s) })
    }
    
    fn no_more_token() -> Result<Node, Error> {
        Err(Self { body: "no more token".to_string() })
    }
}