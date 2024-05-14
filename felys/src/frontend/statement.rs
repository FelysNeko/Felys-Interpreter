use crate::shared::{
    KeywordType as KT, 
    TokenType as TT,
    NodeType as NT,
    ValueType as VT,
    Statement, 
    Error, 
    Node
};
use super::Lexer;

impl Lexer<'_>{
    pub(super) fn parse_next_stat(&mut self) -> Result<Option<Statement>, Error> {
        let next: Option<Statement> = if let Some(tk) = self.token.last() {
            let stat: Statement = match tk.ttype {
                TT::KEYWORD(KT::IF) => self._parse_if()?,
                TT::KEYWORD(KT::LET) => self._parse_let()?,
                TT::KEYWORD(KT::WHILE) => self._parse_while()?,
                TT::KEYWORD(KT::RENDER) => self._parse_render()?,
                TT::KEYWORD(KT::RETURN) => self._parse_return()?,
                _ => self._parse_expr_to_stat()?,
            };
            Some(stat)
        } else {
            None
        };
        Ok(next)
    }

    fn _parse_if(&mut self) -> Result<Statement, Error> {
        self.eat(TT::KEYWORD(KT::IF))?;
        let expr: Node = self.parse_expression()?;
        let body: Vec<Statement> = self._parse_block()?;
        let alter: Option<Box<Statement>> = if let Some(tk) = self.token.last() {
            match tk.ttype {
                TT::KEYWORD(KT::ELIF) => Some(Box::new(self._parse_elif()?)),
                TT::KEYWORD(KT::ELSE) => Some(Box::new(self._parse_else()?)),
                _ => None
            }
        } else {
            None
        };
        Statement::new(KT::IF, expr,  body, alter)
    }

    fn _parse_elif(&mut self) -> Result<Statement, Error> {
        self.eat(TT::KEYWORD(KT::ELIF))?;
        let expr: Node = self.parse_expression()?;
        let body: Vec<Statement> = self._parse_block()?;
        let alter: Option<Box<Statement>> = if let Some(tk) = self.token.last() {
            match tk.ttype {
                TT::KEYWORD(KT::ELIF) => Some(Box::new(self._parse_elif()?)),
                TT::KEYWORD(KT::ELSE) => Some(Box::new(self._parse_else()?)),
                _ => None
            }
        } else {
            None
        };
        Statement::new(KT::IF, expr,  body, alter)
    }

    fn _parse_else(&mut self) -> Result<Statement, Error> {
        self.eat(TT::KEYWORD(KT::ELSE))?;
        let body: Vec<Statement> = self._parse_block()?;
        Statement::new(KT::ELSE, Node::always(), body, None)
    }

    fn _parse_let(&mut self) -> Result<Statement, Error> {
        self.eat(TT::KEYWORD(KT::LET))?;
        let expr: Node = self._parse_assign_target()?;
        let body: Vec<Statement> = if let Some(tk) = self.token.last() {
            if tk.ttype == TT::LBRACE {
                self._parse_block()?
            } else {
                let mut stat: Statement = self._parse_expr_to_stat()?;
                stat.ktype = KT::RETURN;
                vec![stat]
            }
        } else {
            return Error::stat_no_more_token();
        };
        Statement::new(KT::LET, expr, body, None)
    }

    fn _parse_assign_target(&mut self) -> Result<Node, Error> {
        let mut ident: Node = if let Some(tk) = self.token.pop() {
            if tk.ttype == TT::NODE(NT::VALUE(VT::IDENT)) {
                Node::from(tk)?
            } else {
                return Error::expect_ident(tk.value)
            }
        } else {
            return Error::no_more_token()
        };

        if let Some(tk) = self.token.pop() {
            if tk.ttype != TT::NODE(NT::BINOPTR) || tk.value != "=" {
                return Error::expect_assignment(tk.value)
            }
        } else {
            return Error::no_more_token()
        }

        if let Some(tk) = self.token.last() {
            if tk.ttype == TT::PIPE {
                self._ident_to_callable(&mut ident)?;
            }
        }

        Ok(ident)
    }

    fn _ident_to_callable(&mut self, ident: &mut Node) -> Result<(), Error> {
        ident.ntype = NT::CALLABLE;
        self.eat(TT::PIPE)?;

        while let Some(tk) = self.token.last() {
            if tk.ttype == TT::COMMA {
                self.eat(TT::COMMA)?;
            } else if tk.ttype == TT::PIPE {
                self.eat(TT::PIPE)?;
                return Ok(());
            } else {
                let next: Node = self._get_ident()?;
                ident.nodes.push(next);
            }
        }

        Error::no_more_token()?;
        Ok(())
    }

    fn _get_ident(&mut self) -> Result<Node, Error> {
        if let Some(tk) = self.token.pop() {
            if tk.ttype == TT::NODE(NT::VALUE(VT::IDENT)) {
                Node::from(tk)
            } else {
                return Error::expect_ident(tk.value)
            }
        } else {
            return Error::no_more_token()
        }
    }

    fn _parse_while(&mut self) -> Result<Statement, Error> {
        self.eat(TT::KEYWORD(KT::WHILE))?;
        let expr: Node = self.parse_expression()?;
        let body: Vec<Statement> = self._parse_block()?;
        Statement::new(KT::WHILE, expr, body, None)
    }

    fn _parse_render(&mut self) -> Result<Statement, Error> {
        self.eat(TT::KEYWORD(KT::RENDER))?;
        let expr: Node = self.parse_expression()?;
        self.eat(TT::SEMICOL)?;
        Statement::new(KT::RENDER, expr, Vec::new(), None)
    }

    fn _parse_return(&mut self) -> Result<Statement, Error> {
        self.eat(TT::KEYWORD(KT::RETURN))?;
        let expr = self.parse_expression()?;
        self.eat(TT::SEMICOL)?;
        Statement::new(KT::RETURN, expr, Vec::new(), None)
    }

    fn _parse_block(&mut self) -> Result<Vec<Statement>, Error> {
        self.eat(TT::LBRACE)?;
        let mut block: Vec<Statement> = Vec::new();
        while let Some(stat) = self.parse_next_stat()? {
            block.push(stat);

            if let Some(tk) = self.token.last() {
                if tk.ttype == TT::RBRACE {
                    break;
                }
            }
        }
        self.eat(TT::RBRACE)?;
        Ok(block)
    }

    fn _parse_expr_to_stat(&mut self) -> Result<Statement, Error> {
        let expr: Node = self.parse_expression()?;
        self.eat(TT::SEMICOL)?;
        Statement::new(KT::NULL, expr, Vec::new(), None)
    }
}


impl Statement {
    pub fn new(
        ktype: KT,
        expr: Node,
        body: Vec<Statement>,
        alter: Option<Box<Statement>>
    ) -> Result<Self, Error> {
        Ok(Self { ktype, expr, body, alter })
    }
}


impl Node {
    fn always() -> Self {
        Self { 
            ntype: NT::VALUE(VT::BOOLEAN),
            value: "true".to_string(),
            nodes: Vec::new()
         }
    }
}

impl Error {
    fn stat_no_more_token() -> Result<Statement, Error> {
        Err(Self { msg: format!("no more token to parse")})
    }

    fn expect_assignment(s: String) -> Result<Node, Error> {
        Err(Self { msg: format!("expect `=`, but see `{}`", s)})
    }

    fn expect_ident(s: String) -> Result<Node, Error> {
        Err(Self { msg: format!("expect identifier, but see `{}`", s)})
    }
}