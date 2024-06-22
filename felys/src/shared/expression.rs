use crate::shared::statement::Block;
use crate::shared::token::{
    BinoptrType,
    UnaoptrType
};

pub struct BinaryNode {
    pub optr: BinoptrType,
    pub left: Box<Node>,
    pub right: Box<Node>,
}

pub struct UnaryNode {
    pub optr: UnaoptrType,
    pub left: Box<Node>,
    pub right: Box<Node>,
}

pub struct IdentifierNode {
    pub ident: String,
    pub param: Vec<String>
}

pub struct FunctionNode {
    pub param: Vec<String>,
    pub body: Block
}

pub enum Node {
    BIN(BinaryNode),
    UNA(UnaryNode),
    FNC(FunctionNode),
    IDN(IdentifierNode)
}
