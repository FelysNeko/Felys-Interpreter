use crate::shared::{
    Program,
    Error
};
use super::Lexer;


pub fn parse(input: String) -> Result<Program, Error> {
    let mut lxr: Lexer = Lexer {
        chars: input.chars().peekable(),
        token: Vec::new()
    };

    while let Some(tk) = lxr.scan_next_token()? {
        lxr.token.push(tk)
    }

    lxr.token.reverse();

    let mut prog: Program = Program::new();
    while let Some(stat) = lxr.parse_next_stat()? {
        prog.body.push(stat)
    }

    Ok(prog)
}
