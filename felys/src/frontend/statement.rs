use crate::frontend::program::Eat;
use crate::shared::error::Error;
use crate::shared::program::Program;
use crate::shared::statement::{Block, ElifStmt, ElseStmt, IfStmt, ReturnStmt, SimpleStmt, Statement, WhileStmt};
use crate::shared::token::{KT, ST, TT};


impl Program {
    pub fn consume(&mut self) -> Result<Option<Statement>, Error> {
        self.parse_statement()
    }
    
    pub(super) fn parse_statement(&mut self) -> Result<Option<Statement>, Error> {
        if let Some(keyword) = self.tokens.last() {
            let stmt = match keyword.kind {
                TT::Key(KT::If) => self.parse_if()?,
                TT::Key(KT::While) => self.parse_while()?,
                TT::Key(KT::Return) => self.parse_return()?,
                _ => self.parse_simple()?
            };
            Ok(Some(stmt))
        } else { Ok(None) }
    }

    pub(super) fn parse_block(&mut self) -> Result<Block, Error> {
        self.eat(ST::LBrace)?;
        let mut body = Vec::new();
        while let Some(stmt) = self.parse_statement()? {
            body.push(stmt);
            if let Some(token) = self.tokens.last() {
                if token.kind == TT::Sym(ST::RBrace) {
                    break;
                }
            }
        }
        self.eat(ST::RBrace)?;
        Ok(Block::new(body))
    }

    fn parse_if(&mut self) -> Result<Statement, Error> {
        self.eat(KT::If)?;
        let expr = self.parse_expression()?;
        let body = self.parse_block()?;
        let alter = if let Some(token) = self.tokens.last() {
            match token.kind {
                TT::Key(KT::Elif) => Some(Box::new(self.parse_elif()?)),
                TT::Key(KT::Else) => Some(Box::new(self.parse_else()?)),
                _ => None
            }
        } else { None };
        IfStmt::build(expr, body, alter)
    }

    fn parse_elif(&mut self) -> Result<Statement, Error> {
        self.eat(KT::Elif)?;
        let expr = self.parse_expression()?;
        let body = self.parse_block()?;
        let alter = if let Some(token) = self.tokens.last() {
            match token.kind {
                TT::Key(KT::Elif) => Some(Box::new(self.parse_elif()?)),
                TT::Key(KT::Else) => Some(Box::new(self.parse_else()?)),
                _ => None
            }
        } else { None };
        ElifStmt::build(expr, body, alter)
    }

    fn parse_else(&mut self) -> Result<Statement, Error> {
        self.eat(KT::Else)?;
        let body = self.parse_block()?;
        ElseStmt::build(body)
    }

    fn parse_while(&mut self) -> Result<Statement, Error> {
        self.eat(KT::While)?;
        let expr = self.parse_expression()?;
        let body = self.parse_block()?;
        WhileStmt::build(expr, body)
    }

    fn parse_return(&mut self) -> Result<Statement, Error> {
        self.eat(KT::Return)?;
        let expr = self.parse_expression()?;
        self.eat(ST::Semicol)?;
        ReturnStmt::build(expr)
    }

    fn parse_simple(&mut self) -> Result<Statement, Error> {
        let expr = self.parse_expression()?;
        self.eat(ST::Semicol)?;
        SimpleStmt::build(expr)
    }
}
