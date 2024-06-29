use crate::shared::statement::Block;
use crate::shared::token::{TT, BT, UT, VT};
use crate::shared::error::Error;


#[derive(Debug)]
pub struct BinaryNode {
    pub optr: BT,
    pub left: Box<Node>,
    pub right: Box<Node>
}


#[derive(Debug)]
pub struct UnaryNode {
    pub optr: UT,
    pub next: Box<Node>,
}


#[derive(Debug)]
pub struct IdentifierNode {
    pub ident: String,
    pub call: bool,
    pub param: Vec<Node>
}


#[derive(Debug)]
pub struct FunctionNode {
    pub param: Vec<String>,
    pub body: Block
}


#[derive(Debug)]
pub struct LiteralNode {
    pub kind: VT,
    pub value: String
}


#[derive(Debug)]
pub enum Node {
    Bin(BinaryNode),
    Una(UnaryNode),
    Fnc(FunctionNode),
    Idn(IdentifierNode),
    Lit(LiteralNode)
}


impl BinaryNode {
    pub fn build(o: TT, l: Node, r: Node) -> Result<Node, Error> {
        if let TT::Bin(optr) = o {
            Ok(Node::Bin(Self {
                optr,
                left: Box::new(l),
                right: Box::new(r)
            }))
        } else {
            Error::node_building_failed()
        }
    }
}


impl UnaryNode {
    pub fn build(o: TT, n: Node) -> Result<Node, Error> {
        if let TT::Una(optr) = o {
            Ok(Node::Una(Self {
                optr,
                next: Box::new(n),
            }))
        } else {
            Error::node_building_failed()
        }
    }
}

impl LiteralNode {
    pub fn build(k: TT, v: String) -> Result<Node, Error> {
        if let TT::Val(kind) = k {
            Ok(Node::Lit(Self {
                kind,
                value: v,
            }))
        } else {
            Error::node_building_failed()
        }
    }
}


impl FunctionNode {
    pub fn build(p: Vec<String>, b: Block) -> Result<Node, Error> {
        Ok(Node::Fnc(Self { param: p, body: b }))
    }
}


impl IdentifierNode {
    pub fn build(i: String, c: bool, p: Vec<Node>) -> Result<Node, Error> {
        Ok(Node::Idn(Self { ident: i, call: c, param: p}))
    }
}


impl Error {
    fn node_building_failed() -> Result<Node, Error> {
        Err(Self { body: "cannot build node".to_string() })
    }
}