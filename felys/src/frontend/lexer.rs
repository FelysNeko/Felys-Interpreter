use std::process::exit;
use colored::Colorize;
use super::core::Lexer;
use super::core::Token;
use super::core::Node;
use super::core::TokenType as TT;


impl Lexer {
    fn bad_char(&self, i:usize) -> ! {
        println!("");
        println!("{}", self.input);
        for _ in 0..i {
            print!(" ");
        }
        println!("{}", "^".red().bold());
        println!("");
        exit(1);
    }

    pub fn scan(input:String) -> Self {
        let mut lxr = Self {
            input, tokens: vec![Token::new(TT::Null, 0)]
        };

        for (i, ch) in lxr.input.chars().enumerate() {
            let last: &mut Token = lxr.tokens.last_mut().expect("silencer");  

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
                        lxr.tokens.push(new);
                    }
                }
            } else if ch.is_ascii_digit() {
                match last.kind {
                    TT::Identifier |
                    TT::Integer => last.push(ch),
                    _ => {
                        let mut new: Token = Token::new(TT::Integer, i);
                        new.push(ch);
                        lxr.tokens.push(new);
                    }
                }
            } else if ch == '=' {
                if (
                    last.kind == TT::BinaryOperator && 
                    (last.value == "=" || last.value == ">" || last.value == "<")
                ) || (
                    last.kind == TT::UnaryOperator && last.value == "!"
                ) {
                    last.value.push(ch)
                } else {
                    let mut new: Token = Token::new(TT::BinaryOperator, i);
                    new.push(ch);
                    lxr.tokens.push(new);
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
                lxr.tokens.push(new);
            } else if ch == '\'' || ch == '\"' {
                let mut new: Token = Token::new(TT::Null, i);
                new.push(ch);
                lxr.tokens.push(new);
            } else if ch == '!' {
                let mut new: Token = Token::new(TT::UnaryOperator, i);
                new.push(ch);
                lxr.tokens.push(new);
            } else if ch == '*' || ch == '/' || ch == '>' || ch == '<' {
                let mut new: Token = Token::new(TT::BinaryOperator, i);
                new.push(ch);
                lxr.tokens.push(new);
            } else if ch == '(' {
                let mut new: Token = Token::new(TT::OpenParentheses, i);
                new.push(ch);
                lxr.tokens.push(new);
            } else if ch == ')' {
                let mut new: Token = Token::new(TT::CloseParentheses, i);
                new.push(ch);
                lxr.tokens.push(new);
            } else {
                lxr.bad_char(i);
            }
        }

        lxr.tokens.reverse();
        lxr.tokens.pop();
        lxr
    }

    pub fn parse(&mut self) -> Node {
        self.parse_expression()
    }

    fn parse_expression(&mut self) -> Node {
        self.parse_assignment()
    }

    fn parse_assignment(&mut self) -> Node {
        let mut left = self.parse_compare();
        while let Some(tk) = self.tokens.pop() {
            if tk.kind == TT::BinaryOperator && tk.value == "=" {
                let mut new = Node::from(tk);
                new.push(left);
                new.push(self.parse_assignment());
                left = new;
            } else {
                self.tokens.push(tk);
                break;
            }
        }
        left
    }

    fn parse_compare(&mut self) -> Node {
        let mut left = self.parse_additive();
        while let Some(tk) = self.tokens.pop() {
            if tk.kind == TT::BinaryOperator && (
                tk.value == ">" || tk.value == "<" || tk.value == "==" ||
                tk.value == ">=" || tk.value == "<=" || tk.value == "!="
            ) {
                let mut new = Node::from(tk);
                new.push(left);
                new.push(self.parse_additive());
                left = new;
            } else {
                self.tokens.push(tk);
                break;
            }
        }
        left
    }

    fn parse_additive(&mut self) -> Node {
        let mut left = self.parse_multiply();
        while let Some(tk) = self.tokens.pop() {
            if tk.kind == TT::BinaryOperator && (tk.value == "+" || tk.value == "-") {
                let mut new = Node::from(tk);
                new.push(left);
                new.push(self.parse_multiply());
                left = new;
            } else {
                self.tokens.push(tk);
                break;
            }
        }
        left
    }

    fn parse_multiply(&mut self) -> Node {
        let mut left = self.parse_unary();
        while let Some(tk) = self.tokens.pop() {
            if tk.kind == TT::BinaryOperator && (tk.value == "*" || tk.value == "/") {
                let mut new = Node::from(tk);
                new.push(left);
                new.push(self.parse_unary());
                left = new;
            } else {
                self.tokens.push(tk);
                break;
            }
        }
        left
    }

    fn parse_unary(&mut self) -> Node {
        while let Some(tk) = self.tokens.pop() {
            if tk.kind == TT::UnaryOperator {
                let mut new = Node::from(tk);
                new.push(self.parse_unary());
                return new;
            } else {
                self.tokens.push(tk);
                break;
            }
        }
        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Node {
        if let Some(tk) = self.tokens.pop() {
            match tk.kind {
                TT::Identifier |
                TT::Integer |
                TT::String => Node::from(tk),
                TT::OpenParentheses => {
                    let expr = self.parse_expression();
                    self.eat_close_parentheses();
                    expr
                },
                _ => exit(1)
            }
        } else {
            exit(1)
        }
    }

    fn eat_close_parentheses(&mut self) {
        if let Some(cp) = self.tokens.pop() {
            if cp.kind != TT::CloseParentheses {
                exit(1)
            }
        } else {
            exit(1)
        }
    }
}