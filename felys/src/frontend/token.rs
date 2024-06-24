use std::iter::Peekable;
use std::str::Chars;
use crate::shared::{Error, Token, BT, KT, ST, TT, UT, VT};


pub fn tokenize(c: String) -> Result<Vec<Token>, Error> {
    let mut lexer = Lexer {
        chars: c.chars().peekable(),
        buf: Vec::new()
    };

    while let Some(tk) = lexer.scan_next()? {
        lexer.buf.push(tk)
    }
    
    lexer.buf.reverse();
    Ok(lexer.buf)
}

struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
    buf: Vec<Token>
}

impl Lexer<'_> {
    fn scan_next(&mut self) -> Result<Option<Token>, Error> {
        while let Some(ch) = self.chars.peek() {
            if *ch == ' ' || *ch == '\n' || *ch == '\r' {
                self.chars.next();
            } else {
                break;
            }
        }

        let next = if let Some(ch) = self.chars.peek() {
            let token = match ch {
                '\'' |
                '\"' => self.scan_string()?,
                '0'..='9' |
                '.' => self.scan_number()?,
                'a'..='z' |
                'A'..='Z' |
                '_' => self.scan_ident()?,
                '*' |
                '/' |
                '%' => self.scan_simple_binoptr()?,
                '+' |
                '-' => self.scan_additive_optr()?,
                '>' |
                '<' |
                '=' |
                '!' => self.scan_comparative_optr()?,
                '(' |
                ')' |
                '{' |
                '}' |
                ';' |
                ',' => self.scan_symbol()?,
                _ => return Error::lexer_invalid_char(ch)
            };
            Some(token)
        } else {
            None
        };
        Ok(next)
    }

    fn scan_string(&mut self) -> Result<Token, Error> {
        let mut value = String::new();

        let sos = match self.chars.next() {
            Some(ch) => ch,
            None => return Error::lexer_reaches_end()
        };

        for ch in self.chars.by_ref() {
            if ch != sos {
                value.push(ch);
            } else {
                return Ok(Token::new(TT::Val(VT::String), value));
            }
        }

        Error::string_not_closed(value)
    }

    fn scan_number(&mut self) -> Result<Token, Error> {
        let mut value = String::new();
        let mut dot = false;

        while let Some(ch) = self.chars.peek() {
            if ch.is_ascii_digit() || *ch == '.' {
                if *ch == '.' {
                    if dot {
                        return Error::number_two_dots(value);
                    } else {
                        dot = true;
                    }
                }

                value.push(*ch);
                self.chars.next();
            } else {
                break;
            }
        }
        Ok(Token::new(TT::Val(VT::String), value))
    }

    fn scan_ident(&mut self) -> Result<Token, Error> {
        let mut value = String::new();

        while let Some(ch) = self.chars.peek() {
            if ch.is_ascii_alphanumeric() || *ch == '_' {
                value.push(*ch);
                self.chars.next();
            } else {
                break;
            }
        }

        let tt = match value.as_str() {
            "and" => TT::Bin(BT::And),
            "xor" => TT::Bin(BT::Xor),
            "or" => TT::Bin(BT::Or),
            "if" => TT::Key(KT::If),
            "elif" => TT::Key(KT::Elif),
            "else" => TT::Key(KT::Else),
            "while" => TT::Key(KT::While),
            "return" => TT::Key(KT::Return),
            "true" => TT::Val(VT::Boolean),
            "false" => TT::Val(VT::Boolean),
            "none" => TT::Val(VT::None),
            _ => TT::Identifier
        };

        Ok(Token::new(tt, value))
    }

    fn scan_simple_binoptr(&mut self) -> Result<Token, Error> {
        let mut value = String::new();

        if let Some(ch) = self.chars.next() {
            value.push(ch);
        } else {
            return Error::lexer_reaches_end();
        }

        if let Some(eq) = self.chars.peek() {
            if *eq == '=' {
                value.push('=');
                self.chars.next();
            }
        }

        let tt = match value.as_str() {
            "*" => TT::Bin(BT::Mul),
            "/" => TT::Bin(BT::Div),
            "%" => TT::Bin(BT::Mod),
            "*=" => TT::Bin(BT::Mue),
            "/=" => TT::Bin(BT::Die),
            "%=" => TT::Bin(BT::Moe),
            v => return Error::unknown_binary_operator(v)
        };

        Ok(Token::new(tt, value))
    }

    fn scan_additive_optr(&mut self) -> Result<Token, Error> {
        let mut value = String::new();

        if let Some(ch) = self.chars.next() {
            value.push(ch);
        } else {
            return Error::lexer_reaches_end();
        }

        if let Some(eq) = self.chars.peek() {
            if *eq == '=' {
                value.push('=');
                self.chars.next();
            }
        }

        let binary = matches!(self.buf.last(), 
            Some(prev) if matches!(prev.kind, 
                TT::Val(_) | 
                TT::Sym(ST::RParen) | 
                TT::Identifier
            )
        );

        let tt = match (binary, value.as_str()) {
            (true, "+") => TT::Bin(BT::Add),
            (true, "-") => TT::Bin(BT::Sub),
            (true, "+=") => TT::Bin(BT::Ade),
            (true, "-=") => TT::Bin(BT::Sue),
            (false, "+") => TT::Una(UT::Pos),
            (false, "-") => TT::Una(UT::Neg),
            (_, v) => return Error::unknown_binary_operator(v)
        };
        Ok(Token::new(tt, value))
    }

    fn scan_comparative_optr(&mut self) -> Result<Token, Error> {
        let mut value = String::new();

        if let Some(ch) = self.chars.next() {
            value.push(ch);
        } else {
            return Error::lexer_reaches_end();
        }

        if let Some(eq) = self.chars.peek() {
            if *eq == '=' {
                value.push('=');
                self.chars.next();
            }
        }

        let tt = match value.as_str() {
            ">" => TT::Bin(BT::Gt),
            "<" => TT::Bin(BT::Lt),
            "=" => TT::Bin(BT::Asn),
            "!" => TT::Una(UT::Not),
            ">=" => TT::Bin(BT::Le),
            "<=" => TT::Bin(BT::Le),
            "==" => TT::Bin(BT::Eq),
            "!=" => TT::Bin(BT::Ne),
            "=>" => TT::Bin(BT::Arr),
            v => return Error::unknown_binary_operator(v)
        };

        Ok(Token::new(tt, value))
    }
    
    fn scan_symbol(&mut self) -> Result<Token, Error> {
        let ch = match self.chars.next() {
            Some(ch) => ch,
            None => return Error::lexer_reaches_end()
        };
        
        let tt = match ch {
            '(' => TT::Sym(ST::LParen),
            ')' => TT::Sym(ST::RParen),
            '{' => TT::Sym(ST::LBrace),
            '}' => TT::Sym(ST::RBrace),
            ';' => TT::Sym(ST::Semicol),
            ',' => TT::Sym(ST::Comma),
            ch => return Error::unknown_symbol(ch)
        };

        Ok(Token::new(tt, ch.to_string()))
    }
}

impl Error {
    fn lexer_invalid_char(ch: &char) -> Result<Option<Token>, Error> {
        Err(Self { body: format!("char `{}` is unknown", ch) })
    }

    fn lexer_reaches_end() -> Result<Token, Error> {
        Err(Self { body: "no more chars".to_string() })
    }

    fn string_not_closed(s: String) -> Result<Token, Error> {
        Err(Self { body: format!("string `{}` is not closed", s) })
    }

    fn number_two_dots(s: String)  -> Result<Token, Error> {
        Err(Self { body: format!("number `{}` has two decimal points", s) })
    }

    fn unknown_binary_operator(s: &str) -> Result<Token, Error> {
        Err(Self { body: format!("binary operator `{}` is unknown", s) })
    }

    fn unknown_symbol(ch: char) -> Result<Token, Error> {
        Err(Self { body: format!("symbol `{}` is unknown", ch) })
    }
}