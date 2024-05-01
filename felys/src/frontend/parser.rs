use std::process::exit;

use super::Lexer;
use super::Node;
use super::TokenType as TT;


impl Lexer<'_> {
    pub fn _parse(&mut self) -> Node {
        // we want to scan front left to right
        // but `pop()` get you the last element
        // so `reverse()` everything first
        self.tokens.reverse();
        self._parse_expression()
    }

    fn _parse_expression(&mut self) -> Node {
        self._parse_assignment()
    }

    fn _parse_assignment(&mut self) -> Node {
        let mut left: Node = self._parse_logical();

        while let Some(tk) = self.tokens.pop() {
            if tk.kind == TT::ASN {
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
        let mut left: Node = self._parse_compare();

        while let Some(tk) = self.tokens.pop() {
            if tk.kind == TT::AND || tk.kind == TT::OR {
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
            if tk.kind == TT::EQ || tk.kind == TT::NE ||
            tk.kind == TT::SEQ || tk.kind == TT::LEQ ||
            tk.kind == TT::SMR || tk.kind == TT::LGR {
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
            if tk.kind == TT::ADD || tk.kind == TT::SUB {
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
            if tk.kind == TT::MUL || tk.kind == TT::DIV || tk.kind == TT::MOD {
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
            if tk.kind == TT::POS || tk.kind == TT::NEG || tk.kind == TT::NOT {
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
                TT::IDENT |
                TT::NUMBER |
                TT::TRUE | TT::FALSE |
                TT::STRING => Node::from(tk),
                TT::LPAREN => self._parse_parentheses(),
                // expect the arms above, but no show up
                _ => exit(1)
            }
        } else {
            // expect something after operation sign, but no more tokens
            exit(1)
        }
    }

    fn _parse_parentheses(&mut self) -> Node {
        let expr: Node = self._parse_expression();
        
        // eat right parentheses
        if let Some(cp) = self.tokens.pop() {
            if cp.kind != TT::RPAREN {
                // next token is not right parentheses
                exit(1);
            }
        } else {
            // no more tokens
            exit(1);
        }
        expr
    }
}

