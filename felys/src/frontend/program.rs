use crate::frontend::token::tokenize;
use crate::shared::{Error, Program};

impl Program {
    pub fn load(c: String) -> Result<Self, Error> {
        Ok(Self{ tokens: tokenize(c)? })
    }
}
