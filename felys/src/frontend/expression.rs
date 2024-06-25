use crate::frontend::program::Eat;
use crate::shared::token::{BT, ST, TT};
use crate::shared::error::Error;
use crate::shared::expression::{BinaryNode, FunctionNode, IdentifierNode, LiteralNode, Node, UnaryNode};
use crate::shared::program::Program;


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
        if let Some(token) = self.tokens.pop() {
            LiteralNode::build(token.kind, token.value)
        } else {
            Error::no_more_token()
        }
    }
    
    fn parse_identifier(&mut self) -> Result<Node, Error> {
        let ident = match self.tokens.pop() {
            Some(token) => token.value,
            None => return Error::no_more_token()
        };
        
        let mut call = false;
        let param = if let Some(token) = self.tokens.last() {
            if token.kind == TT::Sym(ST::LParen) {
                call = true;
                self.parse_parameter()?
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        };
        
        IdentifierNode::build(ident, call, param)
    }
    
    fn parse_parameter(&mut self) -> Result<Vec<Node>, Error> {
        self.eat(ST::LParen)?;

        let mut param = Vec::new();
        while let Some(token) = self.tokens.last() {
            if token.kind != TT::Sym(ST::RParen) {
                let node = self.parse_expression()?;
                param.push(node);
            } else {
                break;
            }
            
            if let Some(sym) = self.tokens.pop() {
                match sym.kind {
                    TT::Sym(ST::Comma) => (),
                    TT::Sym(ST::RParen) => break,
                    _ => return Error::expect_comma_rparen()
                }
            } else {
                return Error::expect_comma_rparen();
            }
        }
        
        Ok(param)
    }

    fn parse_function(&mut self) -> Result<Node, Error> {
        self.eat(ST::Pipe)?;
        
        let mut param = Vec::new();
        while let Some(token) = self.tokens.pop() {
            match token.kind {
                TT::Identifier => param.push(token.value),
                TT::Sym(ST::Pipe) => break,
                _ => return Error::unexpected_param(token.value)
            }

            if let Some(sym) = self.tokens.pop() {
                match sym.kind {
                    TT::Sym(ST::Comma) => (),
                    TT::Sym(ST::Pipe) => break,
                    _ => return Error::expect_comma_pipe()
                }
            } else {
                return Error::expect_comma_pipe();
            }
        }
        
        let block = if let Some(token) = self.tokens.last() {
            if token.kind == TT::Sym(ST::LBrace) {
                self.parse_block()?
            } else {
                self.parse_statement()?.into()
            }
        } else {
            return Error::no_more_token();
        };
        
        FunctionNode::build(param, block)
    }

    fn parse_lparen(&mut self) -> Result<Node, Error> {
        self.eat(ST::LParen)?;
        let inner = self.parse_expression()?;
        self.eat(ST::RParen)?;
        Ok(inner)
    }
}


impl Error {
    fn token_not_primary(s: &String) -> Result<Node, Error> {
        Err(Self { body: format!("token `{}` is not primary", s) })
    }

    fn unexpected_param(s: String) -> Result<Node, Error> {
        Err(Self { body: format!("token `{}` cannot be a parameter", s) })
    }
    
    fn expect_comma_rparen() -> Result<Vec<Node>, Error> {
        Err(Self { body: "expect `,` or `)`".to_string() })
    }

    fn expect_comma_pipe() -> Result<Node, Error> {
        Err(Self { body: "expect `,` or `|`".to_string() })
    }
}
