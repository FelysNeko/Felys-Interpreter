use crate::shared::expression::Node;

pub struct Block {
    pub body: Vec<Statement>
}

pub struct IfStmt {
    pub expr: Node,
    pub body: Block,
    pub alter: Option<Box<Statement>>
}

pub struct ElifStmt {
    pub expr: Node,
    pub body: Block,
    pub alter: Option<Box<Statement>>
}

pub struct ElseStmt {
    pub body: Block,
}

pub struct WhileStmt {
    pub expr: Node,
    pub body: Block,
}

pub struct ReturnStmt {
    pub expr: Node,
}

pub struct SimpleStmt {
    pub expr: Node,
}

pub enum Statement {
    IF(IfStmt),
    ELIF(ElifStmt),
    ELSE(ElseStmt),
    WHILE(WhileStmt),
    RETURN(ReturnStmt),
    SIMPLE(SimpleStmt)
}
