use super::Lexer;
use super::Token;
use super::Node;
use super::TokenType as TT;
use crate::error::{Msg, Handler};
use crate::error::Handler::{Good, Bad};
use crate::return_if_err;

impl Lexer {
    pub fn parse(input:String) -> Handler<Node, String> {
        let tokens = return_if_err!(scan(&input));
        let mut lxr: Lexer = Self {
            input, tokens
        };
        lxr._parse()
    }


    fn _parse(&mut self) -> Handler<Node, String> {
        self.parse_expression()
    }

    fn parse_expression(&mut self) -> Handler<Node, String> {
        self.parse_assignment()
    }

    fn parse_assignment(&mut self) -> Handler<Node, String> {
        let mut left: Node = match self.parse_logical() {
            Good(n) => n,
            Bad(b) => { return Bad(b); }
        };

        while let Some(tk) = self.tokens.pop() {
            if tk.kind == TT::BinaryOperator && tk.value == "=" {
                let mut new: Node = Node::from(tk);
                let right: Node = match self.parse_assignment() {
                    Good(n) => n,
                    Bad(b) => { return Bad(b); }
                };
                new.push(left);
                new.push(right);
                left = new;
            } else {
                self.tokens.push(tk);
                break;
            }
        }
        Good(left)
    }

    fn parse_logical(&mut self) -> Handler<Node, String> {
        let mut left: Node = return_if_err!(self.parse_bitwise());

        while let Some(tk) = self.tokens.pop() {
            if tk.kind == TT::BinaryOperator && (tk.value == "&&" || tk.value == "||") {
                let mut new: Node = Node::from(tk);
                let right: Node = return_if_err!(self.parse_bitwise());
                new.push(left);
                new.push(right);
                left = new;
            } else {
                self.tokens.push(tk);
                break;
            }
        }
        Good(left)
    }

    fn parse_bitwise(&mut self) -> Handler<Node, String> {
        let mut left: Node = return_if_err!(self.parse_compare());

        while let Some(tk) = self.tokens.pop() {
            if tk.kind == TT::BinaryOperator && (tk.value == "&" || tk.value == "|") {
                let mut new: Node = Node::from(tk);
                let right: Node = return_if_err!(self.parse_compare());
                new.push(left);
                new.push(right);
                left = new;
            } else {
                self.tokens.push(tk);
                break;
            }
        }
        Good(left)
    }

    fn parse_compare(&mut self) -> Handler<Node, String> {
        let mut left: Node = return_if_err!(self.parse_additive());

        while let Some(tk) = self.tokens.pop() {
            if tk.kind == TT::BinaryOperator && (
                tk.value == ">" || tk.value == "<" || tk.value == "==" ||
                tk.value == ">=" || tk.value == "<=" || tk.value == "!="
            ) {
                let mut new: Node = Node::from(tk);
                let right: Node = return_if_err!(self.parse_additive());
                new.push(left);
                new.push(right);
                left = new;
            } else {
                self.tokens.push(tk);
                break;
            }
        }
        Good(left)
    }

    fn parse_additive(&mut self) -> Handler<Node, String> {
        let mut left: Node = return_if_err!(self.parse_multiply());

        while let Some(tk) = self.tokens.pop() {
            if tk.kind == TT::BinaryOperator && (tk.value == "+" || tk.value == "-") {
                let mut new: Node = Node::from(tk);
                let right: Node = return_if_err!(self.parse_multiply());
                new.push(left);
                new.push(right);
                left = new;
            } else {
                self.tokens.push(tk);
                break;
            }
        }
        Good(left)
    }

    fn parse_multiply(&mut self) -> Handler<Node, String> {
        let mut left: Node = return_if_err!(self.parse_unary());

        while let Some(tk) = self.tokens.pop() {
            if tk.kind == TT::BinaryOperator && (
                tk.value == "*" || tk.value == "/" || tk.value == "%"
            ) {
                let mut new: Node = Node::from(tk);
                let right: Node = return_if_err!(self.parse_unary());
                new.push(left);
                new.push(right);
                left = new;
            } else {
                self.tokens.push(tk);
                break;
            }
        }
        Good(left)
    }

    fn parse_unary(&mut self) -> Handler<Node, String> {
        while let Some(tk) = self.tokens.pop() {
            if tk.kind == TT::UnaryOperator {
                let mut new: Node = Node::from(tk);
                let node: Node = return_if_err!(self.parse_unary());
                new.push(node);
                return Good(new);
            } else {
                self.tokens.push(tk);
                break;
            }
        }
        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Handler<Node, String> {
        if let Some(tk) = self.tokens.pop() {
            match tk.kind {
                TT::Identifier |
                TT::Integer |
                TT::String => Good(Node::from(tk)),
                TT::OpenParentheses => self.parse_parentheses(),
                _ => Bad(Msg::not_primary_token(&self, tk))
            }
        } else {
            Bad(Msg::nothing_to_parse(self))
        }
    }

    fn parse_parentheses(&mut self) -> Handler<Node, String> {
        let expr: Node = return_if_err!(self.parse_expression());
        
        if let Some(cp) = self.tokens.pop() {
            if cp.kind != TT::CloseParentheses {
                return Bad(Msg::expected_close_parentheses(self, cp));
            }
        } else {
            return Bad(Msg::nothing_to_parse(self));
        }
        Good(expr)
    }
}


fn scan(input: &String) -> Handler<Vec<Token>, String> {
    let mut tokens = vec![Token::new(TT::Null, 0)];

    for (i, ch) in input.chars().enumerate() {
        let last: &mut Token = match tokens.last_mut() {
            Some(tk) => tk,
            None => { return Bad(Msg::unknown()); }
        };  

        if last.kind == TT::Null && last.value.len() > 0 {
            if last.value.starts_with(ch) {
                last.value.remove(0);
                last.kind = TT::String;
            } else {
                last.value.push(ch);
            }
            continue;
        }

        if ch == ' ' {
            continue;
        }
        
        if ch.is_ascii_alphabetic() || ch == '_' {
            match last.kind {
                TT::Identifier => last.push(ch),
                _ => {
                    let mut new: Token = Token::new(TT::Identifier, i);
                    new.push(ch);
                    tokens.push(new);
                }
            }
        } else if ch.is_ascii_digit() {
            match last.kind {
                TT::Identifier |
                TT::Integer => last.push(ch),
                _ => {
                    let mut new: Token = Token::new(TT::Integer, i);
                    new.push(ch);
                    tokens.push(new);
                }
            }
        } else if ch == '=' {
            if (
                last.kind == TT::BinaryOperator && 
                (last.value == "=" || last.value == ">" || last.value == "<")
            ) || (
                last.kind == TT::UnaryOperator && last.value == "!"
            ) {
                last.kind = TT::BinaryOperator;
                last.value.push(ch)
            } else {
                let mut new: Token = Token::new(TT::BinaryOperator, i);
                new.push(ch);
                tokens.push(new);
            }
        } else if ch == '+' || ch == '-' {
            let mut new: Token = match last.kind {
                TT::BinaryOperator |
                TT::UnaryOperator |
                TT::OpenParentheses |
                TT::Null => Token::new(TT::UnaryOperator, i),
                _ => Token::new(TT::BinaryOperator, i)
            };
            new.push(ch);
            tokens.push(new);
        } else if ch == '&' || ch == '|' { 
            if last.kind == TT::BinaryOperator && (last.value == "&" || last.value == "|") {
                last.push(ch)
            } else {
                let mut new: Token = Token::new(TT::BinaryOperator, i);
                new.push(ch);
                tokens.push(new);
            }
        } else if ch == '\'' || ch == '\"' {
            let mut new: Token = Token::new(TT::Null, i);
            new.push(ch);
            tokens.push(new);
        } else if ch == '!' || ch == '~' {
            let mut new: Token = Token::new(TT::UnaryOperator, i);
            new.push(ch);
            tokens.push(new);
        } else if ch == '*' || ch == '/' || ch == '>' || ch == '<' || ch == '^' || ch == '%' {
            let mut new: Token = Token::new(TT::BinaryOperator, i);
            new.push(ch);
            tokens.push(new);
        } else if ch == '(' {
            let mut new: Token = Token::new(TT::OpenParentheses, i);
            new.push(ch);
            tokens.push(new);
        } else if ch == ')' {
            let mut new: Token = Token::new(TT::CloseParentheses, i);
            new.push(ch);
            tokens.push(new);
        } else {
            return Bad(Msg::lexer_invalid_char(input, i));
        }
    }
    tokens.reverse();
    tokens.pop();
    Good(tokens)
}
