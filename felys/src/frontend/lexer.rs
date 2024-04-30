use std::process::exit;

use super::Lexer;
use super::Token;
use super::Node;
use super::TokenType as TT;

impl Lexer {
    pub fn parse(input:String) -> Node {
        let mut lxr: Lexer = Self {
            input, tokens: vec![Token::new(TT::Null, 0)]
        };
        lxr._scan();
        lxr._parse()
    }
}

impl Lexer {
    fn _parse(&mut self) -> Node {
        self._parse_expression()
    }

    fn _parse_expression(&mut self) -> Node {
        self._parse_assignment()
    }

    fn _parse_assignment(&mut self) -> Node {
        let mut left: Node = self._parse_logical();

        while let Some(tk) = self.tokens.pop() {
            if tk.kind == TT::BinaryOperator && tk.value == "=" {
                let mut new: Node = Node::from(tk);
                let right: Node = self._parse_assignment();
                new.push(left);
                new.push(right);
                left = new;
            } else {
                self.tokens.push(tk);
                break;
            }
        }
        left
    }

    fn _parse_logical(&mut self) -> Node {
        let mut left: Node = self._parse_bitwise();

        while let Some(tk) = self.tokens.pop() {
            if tk.kind == TT::BinaryOperator && (tk.value == "&&" || tk.value == "||") {
                let mut new: Node = Node::from(tk);
                let right: Node = self._parse_bitwise();
                new.push(left);
                new.push(right);
                left = new;
            } else {
                self.tokens.push(tk);
                break;
            }
        }
        left
    }

    fn _parse_bitwise(&mut self) -> Node {
        let mut left: Node = self._parse_compare();

        while let Some(tk) = self.tokens.pop() {
            if tk.kind == TT::BinaryOperator && (tk.value == "&" || tk.value == "|") {
                let mut new: Node = Node::from(tk);
                let right: Node = self._parse_compare();
                new.push(left);
                new.push(right);
                left = new;
            } else {
                self.tokens.push(tk);
                break;
            }
        }
        left
    }

    fn _parse_compare(&mut self) -> Node {
        let mut left: Node = self._parse_additive();

        while let Some(tk) = self.tokens.pop() {
            if tk.kind == TT::BinaryOperator && (
                tk.value == ">" || tk.value == "<" || tk.value == "==" ||
                tk.value == ">=" || tk.value == "<=" || tk.value == "!="
            ) {
                let mut new: Node = Node::from(tk);
                let right: Node = self._parse_additive();
                new.push(left);
                new.push(right);
                left = new;
            } else {
                self.tokens.push(tk);
                break;
            }
        }
        left
    }

    fn _parse_additive(&mut self) -> Node {
        let mut left: Node = self._parse_multiply();

        while let Some(tk) = self.tokens.pop() {
            if tk.kind == TT::BinaryOperator && (tk.value == "+" || tk.value == "-") {
                let mut new: Node = Node::from(tk);
                let right: Node = self._parse_multiply();
                new.push(left);
                new.push(right);
                left = new;
            } else {
                self.tokens.push(tk);
                break;
            }
        }
        left
    }

    fn _parse_multiply(&mut self) -> Node {
        let mut left: Node = self._parse_unary();

        while let Some(tk) = self.tokens.pop() {
            if tk.kind == TT::BinaryOperator && (
                tk.value == "*" || tk.value == "/" || tk.value == "%"
            ) {
                let mut new: Node = Node::from(tk);
                let right: Node = self._parse_unary();
                new.push(left);
                new.push(right);
                left = new;
            } else {
                self.tokens.push(tk);
                break;
            }
        }
        left
    }

    fn _parse_unary(&mut self) -> Node {
        while let Some(tk) = self.tokens.pop() {
            if tk.kind == TT::UnaryOperator {
                let mut new: Node = Node::from(tk);
                let node: Node = self._parse_unary();
                new.push(node);
                return new;
            } else {
                self.tokens.push(tk);
                break;
            }
        }
        self._parse_primary()
    }

    fn _parse_primary(&mut self) -> Node {
        if let Some(tk) = self.tokens.pop() {
            match tk.kind {
                TT::Identifier |
                TT::Integer |
                TT::String => Node::from(tk),
                TT::OpenParentheses => self._parse_parentheses(),
                _ => exit(1)
            }
        } else {
            exit(1)
        }
    }

    fn _parse_parentheses(&mut self) -> Node {
        let expr: Node = self._parse_expression();
        
        if let Some(cp) = self.tokens.pop() {
            if cp.kind != TT::CloseParentheses {
                exit(1);
            }
        } else {
            exit(1);
        }
        expr
    }
}


impl Lexer {
    fn _scan(&mut self) {    
        for (i, ch) in self.input.chars().enumerate() {
            let prev: &mut Token = match self.tokens.last_mut() {
                Some(tk) => tk,
                None => exit(1)
            };
    
            if prev.kind == TT::Null && prev.value.len() > 0 {
                if prev.value.starts_with(ch) {
                    prev.value.remove(0);
                    prev.kind = TT::String;
                } else {
                    prev.value.push(ch);
                }
                continue;
            }
    
            if ch == ' ' {
                continue;
            }
            
            if ch.is_ascii_alphabetic() || ch == '_' {
                match prev.kind {
                    TT::Identifier => prev.push(ch),
                    _ => {
                        let mut new: Token = Token::new(TT::Identifier, i);
                        new.push(ch);
                        self.tokens.push(new);
                    }
                }
            } else if ch.is_ascii_digit() {
                match prev.kind {
                    TT::Identifier |
                    TT::Integer => prev.push(ch),
                    _ => {
                        let mut new: Token = Token::new(TT::Integer, i);
                        new.push(ch);
                        self.tokens.push(new);
                    }
                }
            } else if ch == '=' {
                if (
                    prev.kind == TT::BinaryOperator && 
                    (prev.value == "=" || prev.value == ">" || prev.value == "<")
                ) || (
                    prev.kind == TT::UnaryOperator && prev.value == "!"
                ) {
                    prev.kind = TT::BinaryOperator;
                    prev.value.push(ch)
                } else {
                    let mut new: Token = Token::new(TT::BinaryOperator, i);
                    new.push(ch);
                    self.tokens.push(new);
                }
            } else if ch == '+' || ch == '-' {
                let kind: TT = match prev.kind {
                    TT::BinaryOperator |
                    TT::UnaryOperator |
                    TT::OpenParentheses |
                    TT::Null => TT::UnaryOperator,
                    _ => TT::BinaryOperator
                };
                let mut new: Token = Token::new(kind, i);
                new.push(ch);
                self.tokens.push(new);
            } else if ch == '&' || ch == '|' { 
                if prev.kind == TT::BinaryOperator && (prev.value == "&" || prev.value == "|") {
                    prev.push(ch)
                } else {
                    let mut new: Token = Token::new(TT::BinaryOperator, i);
                    new.push(ch);
                    self.tokens.push(new);
                }
            } else {
                let kind: TT = match ch {
                    '(' => TT::OpenParentheses,
                    ')' => TT::CloseParentheses,
                    '\'' |
                    '\"' => TT::Null,
                    '!' |
                    '~' => TT::UnaryOperator,
                    '*' | '/' | '%' |
                    '>' | '<' |
                    '^' => TT::BinaryOperator,
                    _ => exit(1)
                };
                let mut new: Token = Token::new(kind, i);
                new.push(ch);
                self.tokens.push(new);
            }
        }
        self.tokens.reverse();
        self.tokens.pop();
    }
}