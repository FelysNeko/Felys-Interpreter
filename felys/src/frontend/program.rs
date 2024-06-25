use crate::frontend::token::tokenize;
use crate::shared::error::Error;
use crate::shared::program::Program;

impl Program {
    pub fn load(c: String) -> Result<Self, Error> {
        Ok(Self{ tokens: tokenize(c)? })
    }
}
