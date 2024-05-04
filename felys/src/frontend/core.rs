use crate::core::{
    Program,
    Error
};
use crate::core::frontend::{
    Statement,
    Lexer
};


impl Lexer<'_> {
    pub fn parse(input: String) -> Result<Program, Error> {
        let mut lxr: Lexer<'_> = Lexer {
            iter: input.chars().peekable(),
            tokens: Vec::new()
        };

        while let Some(tk) = lxr.scan_next()? {
            lxr.tokens.push(tk);
        };

        // we want to scan front left to right
        // but `pop()` get you the last element
        // so `reverse()` everything first
        lxr.tokens.reverse();

        let mut main: Program = Program::new();
        while let Some(stat) = lxr.parse_next()? {
            main.push(stat);
        }
        Ok(main)
    }
}


impl Program {
    pub fn new() -> Self {
        Self { body: Vec::new() }
    }

    pub fn push(&mut self, stat: Statement) {
        self.body.push(stat);
    }
}