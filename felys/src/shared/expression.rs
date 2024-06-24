use crate::shared::statement::Block;
use crate::shared::token::{BinoptrType, TokenType, UnaoptrType};
use crate::shared::{Error, TT};

pub struct BinaryNode {
    pub optr: BinoptrType,
    pub left: Box<Node>,
    pub right: Box<Node>
}

pub struct UnaryNode {
    pub optr: UnaoptrType,
    pub next: Box<Node>,
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
    Bin(BinaryNode),
    Una(UnaryNode),
    Fnc(FunctionNode),
    Idn(IdentifierNode)
}

impl BinaryNode {
    pub fn build(o: TokenType, l: Node, r: Node) -> Result<Node, Error> {
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
    pub fn build(o: TokenType, n: Node) -> Result<Node, Error> {
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

impl Error {
    fn node_building_failed() -> Result<Node, Error> {
        Err(Self { body: "cannot build node".to_string() })
    }
}