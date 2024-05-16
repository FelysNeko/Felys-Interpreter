use crate::shared::{
    ValueType as VT,
    NodeType as NT,
    Value,
    Error,
    Node
};


impl Node {
    pub(super) fn eval(&self) -> Result<Value, Error> {
        match self.ntype {
            NT::VALUE(VT::IDENT) => todo!(),
            NT::VALUE(_) => todo!(),
            NT::BINOPTR => todo!(),
            NT::UNAOPTR => todo!(),
            NT::CALLABLE => todo!()
        }
    }
}