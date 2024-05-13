use crate::shared::Error;
use super::Lexer;

pub fn parse(input: String) -> Result<(), Error> {
    let mut lxr: Lexer = Lexer {
        chars: input.chars().peekable(),
        token: Vec::new()
    };

    Ok(())
}

