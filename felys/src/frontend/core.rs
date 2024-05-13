use crate::shared::Error;
use super::Lexer;

pub fn parse(input: String) -> Result<(), Error> {
    let mut lxr: Lexer = Lexer {
        chars: input.chars().peekable(),
        token: Vec::new()
    };

    while let Some(tk) = lxr.next_token()? {
        lxr.token.push(tk)
    }

    Ok(())
}

