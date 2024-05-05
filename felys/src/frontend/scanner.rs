use crate::core::Error;
use crate::core::frontend::{
    TokenType as TT,
    Lexer,
    Token
};


impl Lexer<'_> {
    pub(super) fn scan_next(&mut self) -> Result<Option<Token>, Error> {
        // eat spaces
        while let Some(ch) = self.iter.peek() {
            if *ch == ' ' || *ch == '\n' || *ch == '\r' {
                self.iter.next();
            } else {
                break;
            }
        }

        // `peek()` the next char and decide which category it belongs to
        // all of them will be handled differently
        let next: Option<Token> = if let Some(ch) = self.iter.peek() {
            let token: Token = match ch {
                '\'' |
                '\"' => self._scan_string()?,
                '0'..='9' |
                '.' => self._scan_number()?,
                'a'..='z' |
                'A'..='Z' |
                '_' => self._scan_ident_and_reserved()?,
                '*' |
                '/' |
                '%' => self._scan_simple_binoptr()?,
                '+' |
                '-' => self._scan_add_binoptr()?,
                '>' |
                '<' |
                '=' => self._scan_comp_binoptr()?,
                '!' => self._scan_unaoptr()?,
                '(' |
                ')' |
                '{' |
                '}' | 
                ';' => self._scan_simple_single()?,
                _ => return Error::invalid_char(*ch)
            };
            Some(token)
        } else {
            None
        };
        Ok(next)
    }

    fn _scan_string(&mut self) -> Result<Token, Error> {
        let mut token: Token = Token::string();

        // first char scanned is also the end of string symbol
        let eos = match self.iter.next() {
            Some(ch) => ch,
            None => return Error::no_more_char()
        };

        // no need to `peek()` since we need to eat eos anyway
        while let Some(ch) = self.iter.next() {
            if ch != eos {
                token.push(ch);
            } else {
                return Ok(token);
            }
        }

        // scanning fail if end of string symbol does not show up
        Error::string_not_closed(&token.value)
    }

    fn _scan_number(&mut self) -> Result<Token, Error> {
        let mut token: Token = Token::number();
        let mut dot: bool = false;

        while let Some(ch) = self.iter.peek() {
            // continue scanning if next char is digit or `.`
            if ch.is_ascii_digit() || *ch == '.' {
                // dot can only show up once
                if *ch == '.' {
                    if dot {
                        return Error::two_decimal_points(&token.value);
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
        Ok(token)
    }

    fn _scan_ident_and_reserved(&mut self) -> Result<Token, Error> {
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
            "while" => token.to(TT::WHILE),
            "print" => token.to(TT::PRINT),
            "true" |
            "false" => token.to(TT::BOOLEAN),
            "and" |
            "or" => token.to(TT::BINOPTR),
            _ => (),
        }
        Ok(token)
    }

    fn _scan_simple_binoptr(&mut self) -> Result<Token, Error> {
        if let Some(ch) = self.iter.next() {
            let mut token: Token = Token::binoptr();
            token.push(ch);
            Ok(token)
        } else {
            Error::no_more_char()
        }
    }

    fn _scan_add_binoptr(&mut self) -> Result<Token, Error> {
        // this decides whether an additive sign is a unary or binary operator
        // type `TT::NULL` is assigned to `prev` when nothing yet in `self.tokens`
        // in other word, this token is the first token of the input
        let prev: TT = match self.tokens.last() {
            Some(tk) => tk.kind,
            None => TT::NULL
        };

        if let Some(ch) = self.iter.next() {
            let mut token: Token = Token::binoptr();
            match prev {
                TT::BINOPTR |
                TT::UNAOPTR |
                TT::LPAREN |
                TT::NULL => token.to(TT::UNAOPTR),
                _ => ()
                
            };
            token.push(ch);
            Ok(token)
        } else {
            Error::no_more_char()
        }
    }

    fn _scan_comp_binoptr(&mut self) -> Result<Token, Error> {
        if let Some(ch) = self.iter.next() {
            let mut token: Token = Token::binoptr();
            token.push(ch);

            // scan one more char to see if it is `=`
            // if yes, also eat and push to this token
            if let Some(ch) = self.iter.peek() {
                if *ch == '=' {
                    token.push(*ch);
                    self.iter.next();
                }
            }
            Ok(token)
        } else {
            Error::no_more_char()
        }
    }

    fn _scan_unaoptr(&mut self) -> Result<Token, Error> {
        if let Some(ch) = self.iter.next() {
            let mut token: Token = Token::unaoptr();
            token.push(ch);

            // scan one more char to see if it is `=`
            // if yes, also eat and push to this token
            if let Some(ch) = self.iter.peek() {
                if *ch == '=' {
                    token.push(*ch);
                    self.iter.next();
                }
            }
            Ok(token)
        } else {
            Error::no_more_char()
        }
    }

    fn _scan_simple_single(&mut self) -> Result<Token, Error> {
        if let Some(ch) = self.iter.next() {
            let mut token = Token::null();
            token.push(ch);
            match ch {
                '(' => token.to(TT::LPAREN),
                ')' => token.to(TT::RPAREN),
                '{' => token.to(TT::LBRACE),
                '}' => token.to(TT::RBRACE),
                ';' => token.to(TT::SEMICOL),
                _ => ()
            }
            Ok(token)
        } else {
            Error::no_more_char()
        }
    }
}


impl Token {
    pub fn null() -> Self {
        Self::new(TT::NULL)
    }

    pub fn unaoptr() -> Self {
        Self::new(TT::UNAOPTR)
    }

    pub fn binoptr() -> Self {
        Self::new(TT::BINOPTR)
    }

    pub fn identifier() -> Self {
        Self::new(TT::IDENT)
    }

    pub fn number() -> Self {
        Self::new(TT::NUMBER)
    }

    pub fn string() -> Self {
        Self::new(TT::STRING)
    }

    pub fn new(kind: TT) -> Self {
        Self {
            kind,
            value: String::new(),
        }
    }

    pub fn push(&mut self, ch:char) {
        self.value.push(ch);
    }

    pub fn to(&mut self, kind:TT) {
        self.kind = kind;
    }
}


impl Error {
    fn string_not_closed(value: &String) -> Result<Token, Error> {
        Err(Self {
            msg: format!("string `{}` is not closed", value)
        })
    }

    fn two_decimal_points(value: &String) -> Result<Token, Error> {
        Err(Self {
            msg: format!("`{}` has two decimal points", value)
        })
    }

    fn no_more_char() -> Result<Token, Error> {
        Err(Self {
            msg: format!("expect at least one more char, but no more")
        })
    }

    fn invalid_char(ch: char) -> Result<Option<Token>, Error> {
        Err(Self {
            msg: format!("invalid char `{}`", ch)
        })
    }
}