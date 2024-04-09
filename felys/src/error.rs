use colored::Colorize;
use crate::{
    frontend::{Lexer, Node, Token},
    runtime::Value 
};


pub enum Handler<T, M> {
    Good(T),
    Bad(M)
}

pub struct Msg;

#[macro_export]
macro_rules! return_if_err {
    ($e:expr) => {
        match $e {
            Good(n) => n,
            Bad(b) => { return Bad(b); }
        }
    };
}


impl Msg {
    pub fn unknown() -> String {
        format!(
            "{}: but message not implented",
            "Error".red()
        )
    }

    pub fn variable_not_defined(input:String) -> String {
        format!(
            "{}: the variable `{}` is not defined",
            "Error".red(), input.red()
        )
    }

    pub fn binary_operation_not_implented(n:&Node) -> String {
        format!(
            "{}: have not implement `{}` binary operation",
            "Error".red(), n.value.red()
        )
    }

    pub fn unary_operation_not_implented(n:&Node) -> String {
        format!(
            "{}: have not implement `{}` unary operation",
            "Error".red(), n.value.red()
        )
    }

    pub fn cannot_parse_to_isize(val:&Value) -> String {
        format!(
            "{}: cannot parse `{}` to isize",
            "Error".red(), val.value.red()
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
        for _ in 0..lxr.input.len() {
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

