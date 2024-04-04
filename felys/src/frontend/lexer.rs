use std::process::exit;
use colored::Colorize;
use super::core::Lexer;
use super::core::Token;
use super::core::Node;
use super::core::TokenType as TT;


impl Lexer {
    fn bad_char(&self, i:usize) -> ! {
        println!("");
        println!("{}", self.raw);
        for _ in 0..i {
            print!(" ");
        }
        println!("{}", "^".red());
        println!("");
        exit(1);
    }

    pub fn scan(raw:String) -> Self {
        let mut lxr = Self {
            raw, data: vec![Token::new(TT::Null, 0)]
        };

        for (i, ch) in lxr.raw.chars().enumerate() {
            let last: &mut Token = lxr.data.last_mut().expect("silencer");
            
            if ch == ' ' {
                continue;
            } else if ch.is_ascii_alphabetic() {
                match last.kind {
                    TT::Identifier => last.push(ch),
                    _ => {
                        let mut new: Token = Token::new(TT::Identifier, i);
                        new.push(ch);
                        lxr.data.push(new);
                    }
                }
            } else if ch.is_ascii_digit() {
                match last.kind {
                    TT::Identifier |
                    TT::Number => last.push(ch),
                    _ => {
                        let mut new: Token = Token::new(TT::Number, i);
                        new.push(ch);
                        lxr.data.push(new);
                    }
                }
            } else {
                lxr.bad_char(i);
            }
        }

        lxr.data.reverse();
        lxr.data.pop();
        lxr
    }

    pub fn parse(&mut self) -> Node {
        Node::from(Token::new(TT::Identifier, 0))
    }
}