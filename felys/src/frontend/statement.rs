use crate::shared::error::Error;
use crate::shared::program::Program;
use crate::shared::statement::{Block, Statement};


impl Program {
    pub fn consume(&mut self) -> Result<Statement, Error> {
        self.parse_statement()
    }
    
    pub(super) fn parse_statement(&mut self) -> Result<Statement, Error> {
        todo!()
    }

    pub(super) fn parse_block(&mut self) -> Result<Block, Error> {
        todo!()
    }

    fn parse_if(&mut self) -> Result<Statement, Error> {
        todo!()
    }

    fn parse_elif(&mut self) -> Result<Statement, Error> {
        todo!()
    }

    fn parse_else(&mut self) -> Result<Statement, Error> {
        todo!()
    }

    fn parse_while(&mut self) -> Result<Statement, Error> {
        todo!()
    }

    fn parse_return(&mut self) -> Result<Statement, Error> {
        todo!()
    }

    fn parse_simple(&mut self) -> Result<Statement, Error> {
        todo!()
    }
}