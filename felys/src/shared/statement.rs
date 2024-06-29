use crate::shared::error::Error;
use crate::shared::expression::Node;


#[derive(Debug)]
pub struct Block {
    pub body: Vec<Statement>
}


#[derive(Debug)]
pub struct IfStmt {
    pub expr: Node,
    pub body: Block,
    pub alter: Option<Box<Statement>>
}


#[derive(Debug)]
pub struct ElifStmt {
    pub expr: Node,
    pub body: Block,
    pub alter: Option<Box<Statement>>
}


#[derive(Debug)]
pub struct ElseStmt {
    pub body: Block,
}


#[derive(Debug)]
pub struct WhileStmt {
    pub expr: Node,
    pub body: Block,
}


#[derive(Debug)]
pub struct ReturnStmt {
    pub expr: Node,
}


#[derive(Debug)]
pub struct SimpleStmt {
    pub expr: Node,
}


#[derive(Debug)]
pub enum Statement {
    If(IfStmt),
    Elif(ElifStmt),
    Else(ElseStmt),
    While(WhileStmt),
    Return(ReturnStmt),
    Simple(SimpleStmt)
}


impl Block {
    pub fn new(body: Vec<Statement>) -> Self {
        Self {body}
    }
}


impl IfStmt {
    pub fn build(expr: Node, body: Block, alter: Option<Box<Statement>>) -> Result<Statement, Error> {
        Ok(Statement::If(IfStmt { expr, body, alter }))
    }
}


impl ElifStmt {
    pub fn build(expr: Node, body: Block, alter: Option<Box<Statement>>) -> Result<Statement, Error> {
        Ok(Statement::Elif(ElifStmt { expr, body, alter }))
    }
}


impl ElseStmt {
    pub fn build(body: Block) -> Result<Statement, Error> {
       Ok(Statement::Else(ElseStmt { body }))
    }
}


impl WhileStmt {
    pub fn build(expr: Node, body: Block) -> Result<Statement, Error> {
       Ok(Statement::While(WhileStmt { expr, body }))
    }
}


impl ReturnStmt {
    pub fn build(expr: Node) -> Result<Statement, Error> {
        Ok(Statement::Return(ReturnStmt { expr }))
    }
}


impl SimpleStmt {
    pub fn build(expr: Node) -> Result<Statement, Error> {
        Ok(Statement::Simple(SimpleStmt { expr }))
    }
}
