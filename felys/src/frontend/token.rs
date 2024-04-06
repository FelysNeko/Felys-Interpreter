use super::Token;
use super::TokenType as TT;

impl Token {
    pub fn new(kind: TT, s:usize) -> Self {
        Self {
            kind,
            value: String::new(),
            loc: (s, s)
        }
    }

    pub fn push(&mut self, ch:char) {
        self.value.push(ch);
        self.loc.1 += 1;
    }
}