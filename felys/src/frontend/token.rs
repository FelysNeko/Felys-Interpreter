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
    pub(super) fn next_token(&mut self) -> Result<Option<Token>, Error> {
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
                '\"' => self._scan_string()?,
                '0'..='9' |
                '.' => self._scan_number()?,
                'a'..='z' |
                'A'..='Z' |
                '_' => self._scan_ident_n_reserved()?,
                '*' |
                '/' |
                '%' => self._scan_simple_binoptr()?,
                '+' |
                '-' => self._scan_add_binoptr()?,
                '>' |
                '<' => self._scan_cmp_binoptr()?,
                '=' => self._scan_assignment()?,
                '!' => self._scan_neg_unaoptr()?,
                '(' => self._scan_left_paren()?,
                ')' |
                '{' |
                '}' |
                ';' |
                ',' => self._scan_simple()?,
                _ => return Error::invalid_char(ch)
            };
            Some(token)
        } else {
            None
        };
        Ok(next)
    }

    fn _scan_string(&mut self) -> Result<Token, Error> {
        let mut token: Token = Token::string();

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

    fn _scan_number(&mut self) -> Result<Token, Error> {
        let mut token: Token = Token::number();
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

    fn _scan_ident_n_reserved(&mut self) -> Result<Token, Error> {
        let mut token: Token = Token::identifier();

        while let Some(ch) = self.chars.peek() {
            if ch.is_ascii_alphabetic() || *ch == '_' {
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
            "and" |
            "or" => TT::NODE(NT::BINOPTR),
            _ => token.ttype
        };
        Ok(token)
    }

    fn _scan_simple_binoptr(&mut self) -> Result<Token, Error> {
        if let Some(ch) = self.chars.next() {
            let mut token: Token = Token::binoptr();
            token.value.push(ch);
            Ok(token)
        } else {
            Error::no_more_char()
        }
    }

    fn _scan_add_binoptr(&mut self) -> Result<Token, Error> {
        let prev: TT = match self.token.last() {
            Some(tk) => tk.ttype,
            None => TT::LPAREN
        };

        if let Some(ch) = self.chars.next() {
            let mut token: Token = Token::binoptr();
            token.value.push(ch);
            token.ttype = match prev {
                TT::NODE(NT::UNAOPTR) |
                TT::NODE(NT::BINOPTR) |
                TT::LPAREN => TT::NODE(NT::UNAOPTR),
                _ => token.ttype
            };
            Ok(token)
        } else {
            Error::no_more_char()
        }
    }

    fn _scan_assignment(&mut self) -> Result<Token, Error> {
        if let Some(ch) = self.chars.next() {
            let mut token: Token = Token::binoptr();
            token.value.push(ch);

            if let Some(ch) = self.chars.peek() {
                if *ch == '>' {
                    token.ttype = TT::ARROW;
                }
                if *ch == '=' || *ch == '>' {
                    token.value.push(*ch);
                    self.chars.next();
                }
            }
            Ok(token)
        } else {
            Error::no_more_char()
        }
    }

    fn _scan_cmp_binoptr(&mut self) -> Result<Token, Error> {
        if let Some(ch) = self.chars.next() {
            let mut token: Token = Token::binoptr();
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

    fn _scan_neg_unaoptr(&mut self) -> Result<Token, Error> {
        if let Some(ch) = self.chars.next() {
            let mut token: Token = Token::unaoptr();
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

    fn _scan_left_paren(&mut self) -> Result<Token, Error> {
        if let Some(ch) = self.chars.next() {
            let mut token: Token = Token::lparen();
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

    fn _scan_simple(&mut self) -> Result<Token, Error> {
        if let Some(ch) = self.chars.next() {
            let ttype: TT = match ch {
                ')' => TT::RPAREN,
                '{' => TT::LBRACE,
                '}' => TT::RBRACE,
                ';' => TT::SEMICOL,
                ',' => TT::COMMA,
                _ => return Error::invalid_single_token(ch)
            };
            let mut token: Token = Token::new(ttype);
            token.value.push(ch);
            Ok(token)
        } else {
            Error::no_more_char()
        }
    }
}


impl Token {
    fn lparen() -> Self {
        Self::new(TT::LPAREN)
    }

    fn unaoptr() -> Self {
        Self::new(TT::NODE(NT::UNAOPTR))
    }

    fn binoptr() -> Self {
        Self::new(TT::NODE(NT::BINOPTR))
    }

    fn identifier() -> Self {
        Self::new(TT::NODE(NT::VALUE(VT::IDENT)))
    }

    fn number() -> Self {
        Self::new(TT::NODE(NT::VALUE(VT::NUMBER)))
    }

    fn string() -> Self {
        Self::new(TT::NODE(NT::VALUE(VT::STRING)))
    }

    fn new(ttype: TT) -> Self {
        Self {
            ttype,
            value: String::new(),
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
        Err(Self { msg: format!("no more char") })
    }
}