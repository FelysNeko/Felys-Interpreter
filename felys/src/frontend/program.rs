use crate::frontend::token::tokenize;
use crate::shared::error::Error;
use crate::shared::expression::Node;
use crate::shared::program::Program;
use crate::shared::token::{ST, TT, KT};


impl Program {
    pub fn load(c: String) -> Result<Self, Error> {
        Ok(Self{
            tokens: tokenize(c)?,
            worker: None
        })
    }
}


pub trait Eat<T> {
    fn eat(&mut self, t: T) -> Result<(), Error>;
}


impl Eat<ST> for Program {
    fn eat(&mut self, t: ST) -> Result<(), Error> {
        if let Some(token) = self.tokens.pop() {
            if token.kind != TT::Sym(t) {
                Error::eat_wrong_token(token.value)?;
            }
        } else {
            Error::no_more_token()?;
        }
        Ok(())
    }
}


impl Eat<KT> for Program {
    fn eat(&mut self, t: KT) -> Result<(), Error> {
        if let Some(token) = self.tokens.pop() {
            if token.kind != TT::Key(t) {
                Error::eat_wrong_token(token.value)?;
            }
        } else {
            Error::no_more_token()?;
        }
        Ok(())
    }
}


impl Error {
    pub(super) fn no_more_token() -> Result<Node, Error> {
        Err(Self { body: "no more token".to_string() })
    }

    fn eat_wrong_token(s: String) -> Result<Node, Error> {
        Err(Self { body: format!("token `{}` is unexpected", s) })
    }
}