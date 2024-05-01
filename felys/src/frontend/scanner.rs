use std::process::exit;

use super::Lexer;
use super::Token;
use super::TokenType as TT;

impl Lexer<'_> {
    pub fn _scan(&mut self) {    
        while let Some(tk) = self._scan_next() {
            self.tokens.push(tk);
        }
    }

    fn _scan_next(&mut self) -> Option<Token> {
        // eat spaces
        while let Some(ch) = self.iter.peek() {
            if *ch == ' ' {
                self.iter.next();
            } else {
                break;
            }
        }

        // `peek()` the next char and decide which category it belongs to
        // all of them will be handled differently
        if let Some(ch) = self.iter.peek() {
            match ch {
                '\'' |
                '\"' => self._scan_string(),
                '0'..='9' |
                '.' => self._scan_number(),
                'a'..='z' |
                'A'..='Z' |
                '_' => self._scan_ident_and_kw(),
                '*' |
                '/' |
                '%' => self._scan_simple_binoptr(),
                '+' |
                '-' => self._scan_add_binoptr(),
                '>' |
                '<' |
                '=' => self._scan_comp_binoptr(),
                '!' => self._scan_unaoptr(),
                '(' |
                ')' |
                '{' |
                '}' => self._scan_simple_single(),
                _ => exit(1)
            }
        } else {
            None
        }
    }

    fn _scan_string(&mut self) -> Option<Token> {
        let mut token: Token = Token::string();

        // first char scanned is also the end of string symbol
        let eos = match self.iter.next() {
            Some(ch) => ch,
            None => exit(1)
        };

        // no need to `peek()` since we need to eat eos anyway
        while let Some(ch) = self.iter.next() {
            if ch != eos {
                token.push(ch);
            } else {
                return Some(token);
            }
        }

        // scanning fail if end of string symbol does not show up
        exit(1);
    }

    fn _scan_number(&mut self) -> Option<Token> {
        let mut token: Token = Token::number();
        let mut dot: bool = false;

        while let Some(ch) = self.iter.peek() {
            // continue scanning if next char is digit or `.`
            if ch.is_ascii_digit() || *ch == '.' {
                // dot can only show up once
                if *ch == '.' {
                    if dot {
                        exit(1)
                    } else {
                        dot = true;
                    }
                }

                token.push(*ch);
                self.iter.next();
            } else {
                // stop scanning
                break;
            }
        }
        Some(token)
    }

    fn _scan_ident_and_kw(&mut self) -> Option<Token> {
        let mut token: Token = Token::identifier();

        while let Some(ch) = self.iter.peek() {
            // continue scanning if next char is letter or `_`
            if ch.is_ascii_alphabetic() || *ch == '_' {
                token.push(*ch);
                self.iter.next();
            } else {
                // stop scanning
                break;
            }
        }

        // set the token type based on content
        match token.value.as_str() {
            "if" => token.to(TT::IF),
            "elif" => token.to(TT::ELIF),
            "else" => token.to(TT::ELSE),
            "true" => token.to(TT::TRUE),
            "false" => token.to(TT::FALSE),
            "while" => token.to(TT::WHILE),
            "and" => token.to(TT::AND),
            "or" => token.to(TT::OR),
            _ => (),
        }
        Some(token)
    }

    fn _scan_simple_binoptr(&mut self) -> Option<Token> {
        if let Some(ch) = self.iter.next() {
            let mut token: Token = Token::null();
            token.push(ch);
            match ch {
                '*' => token.to(TT::MUL),
                '/' => token.to(TT::DIV),
                '%' => token.to(TT::MOD),
                _ => ()
            }
            Some(token)
        } else {
            None
        }
    }

    fn _scan_add_binoptr(&mut self) -> Option<Token> {
        // this decides whether an additive sign is a unary or binary operator
        // type `TT::NULL` is assigned to `prev` nothing yet in `self.tokens`
        // in other word, this token is the first token of the input
        let prev = match self.tokens.last() {
            Some(tk) => tk.kind,
            None => TT::NULL
        };

        if let Some(ch) = self.iter.next() {
            let mut token: Token = Token::null();
            match prev {
                // all binary operator
                // all unary operator
                // left parentheses
                // it's the first token
                TT::ADD | TT::SUB | TT::MUL | TT::DIV | TT::MOD |
                TT::SMR | TT::LGR | TT::SEQ | TT::LEQ | TT::EQ | TT::NE |
                TT::AND | TT::OR |
                TT::POS | TT::NEG | TT::NOT |
                TT::LPAREN |
                TT::NULL => if ch == '+' {
                    token.to(TT::POS)
                } else if ch == '-' {
                    token.to(TT::NEG)
                },
                // otherwise, it's normal binary operator
                _ =>  if ch == '+' {
                    token.to(TT::ADD)
                } else if ch == '-' {
                    token.to(TT::SUB)
                }
            };
            token.push(ch);
            Some(token)
        } else {
            None
        }
    }

    fn _scan_comp_binoptr(&mut self) -> Option<Token> {
        if let Some(ch) = self.iter.next() {
            let mut token: Token = Token::null();
            token.push(ch);

            // scan one more char to see if it is `=`
            // if yes, also eat and push to this token
            if let Some(ch) = self.iter.peek() {
                if *ch == '=' {
                    token.push(*ch);
                    self.iter.next();
                }
            }
            
            match token.value.as_str() {
                "=" => token.to(TT::ASN),
                "<" => token.to(TT::SMR),
                ">" => token.to(TT::LGR),
                "==" => token.to(TT::EQ),
                "<=" => token.to(TT::SEQ),
                ">=" => token.to(TT::LEQ),
                _ => ()
            }
            Some(token)
        } else {
            None
        }
    }

    fn _scan_unaoptr(&mut self) -> Option<Token> {
        if let Some(ch) = self.iter.next() {
            let mut token: Token = Token::null();
            token.push(ch);

            // scan one more char to see if it is `=`
            // if yes, also eat and push to this token
            if let Some(ch) = self.iter.peek() {
                if *ch == '=' {
                    token.push(*ch);
                    self.iter.next();
                }
            }

            match token.value.as_str() {
                "!" => token.to(TT::NOT),
                "!=" => token.to(TT::NE),
                _ => ()
            }
            Some(token)
        } else {
            None
        }
    }

    fn _scan_simple_single(&mut self) -> Option<Token> {
        if let Some(ch) = self.iter.next() {
            let mut token = Token::null();
            token.push(ch);
            match ch {
                '(' => token.to(TT::LPAREN),
                ')' => token.to(TT::RPAREN),
                '{' => token.to(TT::LBRACE),
                '}' => token.to(TT::RBRACE),
                _ => ()
            }
            Some(token)
        } else {
            None
        }
    }
}