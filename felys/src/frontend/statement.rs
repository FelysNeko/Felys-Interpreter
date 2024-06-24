use crate::shared::{Error, Program, Statement};

impl Program {
    pub fn consume(&mut self) -> Result<Statement, Error> {
        self.parse_statement()
    }
    
    fn parse_statement(&mut self) -> Result<Statement, Error> {
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

    fn parse_block(&mut self) -> Result<Vec<Statement>, Error> {
        todo!()
    }
}