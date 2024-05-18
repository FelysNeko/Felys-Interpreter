use crate::shared::{
    TokenType as TT,
    NodeType as NT,
    ValueType as VT,
    KeywordType as KT,
    Error,
    Token
};
use super::Lexer;


impl Lexer<'_> {
    pub(super) fn scan_next_token(&mut self) -> Result<Option<Token>, Error> {
        while let Some(ch) = self.chars.peek() {
            if *ch == ' ' || *ch == '\n' || *ch == '\r' {
                self.chars.next();
            } else {
                break;
            }
        }

        let next: Option<Token> = if let Some(ch) = self.chars.peek() {
            let token: Token = match ch {
                '\'' |
                '\"' => self.scan_string()?,
                '0'..='9' |
                '.' => self.scan_number()?,
                'a'..='z' |
                'A'..='Z' |
                '_' => self.scan_ident_n_reserved()?,
                '*' |
                '/' |
                '%' => self.scan_simple_binoptr()?,
                '+' |
                '-' => self.scan_add_unaoptr()?,
                '>' |
                '<' |
                '=' => self.scan_cmp_binoptr()?,
                '!' => self.scan_neg_unaoptr()?,
                '(' => self.scan_left_paren()?,
                ')' |
                '{' |
                '}' |
                ';' |
                '|' |
                ',' => self.scan_simple()?,
                _ => return Error::invalid_char(ch)
            };
            Some(token)
        } else {
            None
        };
        Ok(next)
    }

    fn scan_string(&mut self) -> Result<Token, Error> {
        let mut token: Token = Token::from(VT::STRING);

        let sos: char = match self.chars.next() {
            Some(ch) => ch,
            None => return Error::no_more_char()
        };

        while let Some(ch) = self.chars.next() {
            if ch != sos {
                token.value.push(ch);
            } else {
                return Ok(token);
            }
        }

        Error::string_not_closed(token.value)
    }

    fn scan_number(&mut self) -> Result<Token, Error> {
        let mut token: Token = Token::from(VT::NUMBER);
        let mut dot: bool = false;

        while let Some(ch) = self.chars.peek() {
            if ch.is_ascii_digit() || *ch == '.' {
                if *ch == '.' {
                    if dot {
                        return Error::two_decimal_points(token.value);
                    } else {
                        dot = true;
                    }
                }

                token.value.push(*ch);
                self.chars.next();
            } else {
                break;
            }
        }
        Ok(token)
    }

    fn scan_ident_n_reserved(&mut self) -> Result<Token, Error> {
        let mut token: Token = Token::from(VT::IDENT);

        while let Some(ch) = self.chars.peek() {
            if ch.is_ascii_alphanumeric() || *ch == '_' {
                token.value.push(*ch);
                self.chars.next();
            } else {
                break;
            }
        }

        token.ttype = match token.value.as_str() {
            "let" => TT::KEYWORD(KT::LET),
            "while" => TT::KEYWORD(KT::WHILE),
            "if" => TT::KEYWORD(KT::IF),
            "elif" => TT::KEYWORD(KT::ELIF),
            "else" => TT::KEYWORD(KT::ELSE),
            "render" => TT::KEYWORD(KT::RENDER),
            "return" => TT::KEYWORD(KT::RETURN),
            "true" |
            "false" => TT::NODE(NT::VALUE(VT::BOOLEAN)),
            "none" => TT::NODE(NT::VALUE(VT::NONE)),
            "and" |
            "or" => TT::NODE(NT::BINOPTR),
            _ => token.ttype
        };
        Ok(token)
    }

    fn scan_simple_binoptr(&mut self) -> Result<Token, Error> {
        if let Some(ch) = self.chars.next() {
            let mut token: Token = Token::from(NT::BINOPTR);
            token.value.push(ch);
            Ok(token)
        } else {
            Error::no_more_char()
        }
    }

    fn scan_add_unaoptr(&mut self) -> Result<Token, Error> {
        let prev: TT = match self.token.last() {
            Some(tk) => tk.ttype,
            None => TT::LPAREN
        };

        if let Some(ch) = self.chars.next() {
            let mut token: Token = Token::from(NT::UNAOPTR);
            token.value.push(ch);
            token.ttype = match prev {
                TT::NODE(NT::VALUE(_)) |
                TT::NODE(NT::CALLABLE) |
                TT::RPAREN => TT::NODE(NT::BINOPTR),
                _ => token.ttype
            };
            Ok(token)
        } else {
            Error::no_more_char()
        }
    }

    fn scan_cmp_binoptr(&mut self) -> Result<Token, Error> {
        if let Some(ch) = self.chars.next() {
            let mut token: Token = Token::from(NT::BINOPTR);
            token.value.push(ch);

            if let Some(ch) = self.chars.peek() {
                if *ch == '=' {
                    token.value.push(*ch);
                    self.chars.next();
                }
            }
            Ok(token)
        } else {
            Error::no_more_char()
        }
    }

    fn scan_neg_unaoptr(&mut self) -> Result<Token, Error> {
        if let Some(ch) = self.chars.next() {
            let mut token: Token = Token::from(NT::UNAOPTR);
            token.value.push(ch);

            if let Some(ch) = self.chars.peek() {
                if *ch == '=' {
                    token.value.push(*ch);
                    self.chars.next();
                }
            }
            Ok(token)
        } else {
            Error::no_more_char()
        }
    }

    fn scan_left_paren(&mut self) -> Result<Token, Error> {
        if let Some(ch) = self.chars.next() {
            let mut token: Token = Token::from(TT::LPAREN);
            token.value.push(ch);

            if let Some(tk) = self.token.last_mut() {
                if tk.ttype == TT::NODE(NT::VALUE(VT::IDENT)) {
                    tk.ttype = TT::NODE(NT::CALLABLE);
                }
            }

            Ok(token)
        } else {
            Error::no_more_char()
        }
    }

    fn scan_simple(&mut self) -> Result<Token, Error> {
        if let Some(ch) = self.chars.next() {
            let ttype: TT = match ch {
                ')' => TT::RPAREN,
                '{' => TT::LBRACE,
                '}' => TT::RBRACE,
                ';' => TT::SEMICOL,
                ',' => TT::COMMA,
                '|' => TT::PIPE,
                _ => return Error::invalid_single_token(ch)
            };
            let mut token: Token = Token::from(ttype);
            token.value.push(ch);
            Ok(token)
        } else {
            Error::no_more_char()
        }
    }
}


impl Error {
    fn invalid_single_token(c: char) -> Result<Token, Error> {
        Err(Self { msg: format!("cannot convert char `{}` to token", c) })
    }

    fn invalid_char(c: &char) -> Result<Option<Token>, Error> {
        Err(Self { msg: format!("char `{}` is invalid", c) })
    }

    fn string_not_closed(s: String) -> Result<Token, Error> {
        Err(Self { msg: format!("string `{}` is not closed", s) })
    }

    fn two_decimal_points(s: String) -> Result<Token, Error> {
        Err(Self { msg: format!("number `{}` has two decimal points", s) })
    }

    fn no_more_char() -> Result<Token, Error> {
        Err(Self { msg: format!("no more char to parse") })
    }
}