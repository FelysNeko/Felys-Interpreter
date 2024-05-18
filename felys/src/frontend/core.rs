use crate::shared::{
    TokenType as TT,
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


impl Lexer<'_> {
    pub(super) fn eat(&mut self, t: TT) -> Result<(), Error>{
        if let Some(tk) = self.token.pop() {
            if tk.ttype != t {
                Error::incorrect_next_token(t, tk.value)
            } else {
                Ok(())
            }
        } else {
            Error::nothing_to_eat(t)
        }
    }
}


impl Error {
    fn incorrect_next_token(e: TT, s: String) -> Result<(), Error> {
        Err(Self { msg: format!("expect `{:?}`, but see `{}`", e, s) })
    }

    fn nothing_to_eat(e: TT) -> Result<(), Error> {
        Err(Self { msg: format!("expect `{:?}`, but nothing to eat", e) })
    }
}