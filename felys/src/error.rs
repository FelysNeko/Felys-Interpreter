use colored::Colorize;
use crate::frontend::{Lexer, Token};

pub enum Handler<T, E> {
    Good(T),
    Bad(E)
}

pub struct Error;

impl Error {
    pub fn unknown() -> String {
        format!(
            "{} but message not implented",
            "Error".red()
        )
    }

    pub fn not_primary_token(lxr:&Lexer, tk:Token) -> String {
        let mut space = String::new();
        for _ in 0..tk.loc.0 {
            space.push(' ');
        }
        let mut arrow = String::new();
        for _ in tk.loc.0..tk.loc.1 {
            arrow.push('^');
        }
        format!(
            "{}\n{}{}\n{}: this is not a literal or identifier",
            lxr.input, space, "^".red().bold(), "Error".red()
        )
    }

    pub fn expected_close_parentheses(lxr:&Lexer, tk:Token) -> String {
        let mut space = String::new();
        for _ in 0..tk.loc.0 {
            space.push(' ');
        }
        let mut arrow = String::new();
        for _ in tk.loc.0..tk.loc.1 {
            arrow.push('^');
        }
        format!(
            "{}\n{}{}\n{}: expected a close parenthese",
            lxr.input, space, "^".red().bold(), "Error".red()
        )
    }
    
    pub fn nothing_to_parse(lxr:&Lexer) -> String {
        let mut space = String::new();
        for _ in 0..lxr.input.len()+1 {
            space.push(' ');
        }
        format!(
            "{}\n{}{}\n{}: nothing to parse",
            lxr.input, space, "^".red().bold(), "Error".red()
        )
    }

    pub fn lexer_invalid_char(input:&String, i:usize) -> String {
        let mut space = String::new();
        for _ in 0..i {
            space.push(' ');
        }
        format!(
            "{}\n{}{}\n{}: invalid character",
            input, space, "^".red().bold(), "Error".red()
        )
    }
}

