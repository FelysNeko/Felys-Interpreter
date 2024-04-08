use super::Lexer;
use super::Token;
use super::Node;
use super::TokenType as TT;
use crate::error::{
    ErrorStack as ES,
    Error as E,
    Handler,
};
use crate::error::Handler::{Good, Bad};

impl Lexer {
    pub fn parse(input:String) -> Option<Node> {
        let tokens = match scan(&input) {
            Good(tks) => tks,
            Bad(_) => { return None; }
        };

        let mut lxr: Lexer = Self {
            input, tokens
        };
        lxr._parse()
    }


    fn _parse(&mut self) -> Option<Node> {
        match self.parse_expression() {
            Some(n) => Some(n),
            None => { return None; }
        }
    }

    fn parse_expression(&mut self) -> Option<Node> {
        match self.parse_assignment() {
            Some(n) => Some(n),
            None => { return None; }
        }
    }

    fn parse_assignment(&mut self) -> Option<Node> {
        let mut left: Node = match self.parse_logical() {
            Some(n) => n,
            None => { return None; }
        };

        while let Some(tk) = self.tokens.pop() {
            if tk.kind == TT::BinaryOperator && tk.value == "=" {
                let mut new: Node = Node::from(tk);
                let right: Node = match self.parse_assignment() {
                    Some(n) => n,
                    None => { return None; }
                };
                new.push(left);
                new.push(right);
                left = new;
            } else {
                self.tokens.push(tk);
                break;
            }
        }
        Some(left)
    }

    fn parse_logical(&mut self) -> Option<Node> {
        let mut left: Node = match self.parse_bitwise() {
            Some(n) => n,
            None => { return None; }
        };

        while let Some(tk) = self.tokens.pop() {
            if tk.kind == TT::BinaryOperator && (tk.value == "&&" || tk.value == "||") {
                let mut new: Node = Node::from(tk);
                let right: Node = match self.parse_bitwise() {
                    Some(n) => n,
                    None => { return None; }
                };
                new.push(left);
                new.push(right);
                left = new;
            } else {
                self.tokens.push(tk);
                break;
            }
        }
        Some(left)
    }

    fn parse_bitwise(&mut self) -> Option<Node> {
        let mut left: Node = match self.parse_compare() {
            Some(n) => n,
            None => { return None; }
        };

        while let Some(tk) = self.tokens.pop() {
            if tk.kind == TT::BinaryOperator && (tk.value == "&" || tk.value == "|") {
                let mut new: Node = Node::from(tk);
                let right: Node = match self.parse_compare() {
                    Some(n) => n,
                    None => { return None; }
                };
                new.push(left);
                new.push(right);
                left = new;
            } else {
                self.tokens.push(tk);
                break;
            }
        }
        Some(left)
    }

    fn parse_compare(&mut self) -> Option<Node> {
        let mut left: Node = match self.parse_additive() {
            Some(n) => n,
            None => { return None; }
        };

        while let Some(tk) = self.tokens.pop() {
            if tk.kind == TT::BinaryOperator && (
                tk.value == ">" || tk.value == "<" || tk.value == "==" ||
                tk.value == ">=" || tk.value == "<=" || tk.value == "!="
            ) {
                let mut new: Node = Node::from(tk);
                let right: Node = match self.parse_additive() {
                    Some(n) => n,
                    None => { return None; }
                };
                new.push(left);
                new.push(right);
                left = new;
            } else {
                self.tokens.push(tk);
                break;
            }
        }
        Some(left)
    }

    fn parse_additive(&mut self) -> Option<Node> {
        let mut left: Node = match self.parse_multiply() {
            Some(n) => n,
            None => { return None; }
        };

        while let Some(tk) = self.tokens.pop() {
            if tk.kind == TT::BinaryOperator && (tk.value == "+" || tk.value == "-") {
                let mut new: Node = Node::from(tk);
                let right: Node = match self.parse_multiply() {
                    Some(n) => n,
                    None => { return None; }
                };
                new.push(left);
                new.push(right);
                left = new;
            } else {
                self.tokens.push(tk);
                break;
            }
        }
        Some(left)
    }

    fn parse_multiply(&mut self) -> Option<Node> {
        let mut left: Node = match self.parse_unary() {
            Some(n) => n,
            None => { return None; }
        };

        while let Some(tk) = self.tokens.pop() {
            if tk.kind == TT::BinaryOperator && (tk.value == "*" || tk.value == "/") {
                let mut new: Node = Node::from(tk);
                let right: Node = match self.parse_unary() {
                    Some(n) => n,
                    None => { return None; }
                };
                new.push(left);
                new.push(right);
                left = new;
            } else {
                self.tokens.push(tk);
                break;
            }
        }
        Some(left)
    }

    fn parse_unary(&mut self) -> Option<Node> {
        while let Some(tk) = self.tokens.pop() {
            if tk.kind == TT::UnaryOperator {
                let mut new: Node = Node::from(tk);
                let node: Node = match self.parse_unary() {
                    Some(n) => n,
                    None => { return None; }
                };
                new.push(node);
                return Some(new);
            } else {
                self.tokens.push(tk);
                break;
            }
        }
        match self.parse_primary() {
            Some(n) => Some(n),
            None => { return None; }
        }
    }

    fn parse_primary(&mut self) -> Option<Node> {
        if let Some(tk) = self.tokens.pop() {
            match tk.kind {
                TT::Identifier |
                TT::Integer |
                TT::String => Some(Node::from(tk)),
                TT::OpenParentheses => self.parse_parentheses(),
                _ => None
            }
        } else {
            None
        }
    }

    fn parse_parentheses(&mut self) -> Option<Node> {
        let expr: Node = match self.parse_expression() {
            Some(n) => n,
            None => { return None; }
        };
        
        if let Some(cp) = self.tokens.pop() {
            if cp.kind != TT::CloseParentheses {
                return None;
            }
        } else {
            return None;
        }
        Some(expr)
    }
}


fn scan(input: &String) -> Handler<Vec<Token>, ES> {
    let mut tokens = vec![Token::new(TT::Null, 0)];

    for (i, ch) in input.chars().enumerate() {
        let last: &mut Token = match tokens.last_mut() {
            Some(tk) => tk,
            None => { return Bad(ES::new(E::unknown())); }
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
            return Bad(ES::new(E::lexer_invalid_char(input, i)));
        }
    }
    tokens.reverse();
    tokens.pop();
    Good(tokens)
}
