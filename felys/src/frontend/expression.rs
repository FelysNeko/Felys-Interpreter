use crate::shared::{Error, Node, Program};

impl Program {
    pub(super) fn parse_expression(&mut self) -> Result<Node, Error> {
        self.parse_assignement()
    }
    
    fn parse_assignement(&mut self) -> Result<Node, Error> {
        todo!()
    }

    fn parse_logical(&mut self) -> Result<Node, Error> {
        todo!()
    }

    fn parse_compare(&mut self) -> Result<Node, Error> {
        todo!()
    }

    fn parse_additive(&mut self) -> Result<Node, Error> {
        todo!()
    }

    fn parse_multiply(&mut self) -> Result<Node, Error> {
        todo!()
    }

    fn parse_unary(&mut self) -> Result<Node, Error> {
        todo!()
    }

    fn parse_primary(&mut self) -> Result<Node, Error> {
        todo!()
    }
}