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
                TT::KEYWORD(KT::IF) => self.parse_if()?,
                TT::KEYWORD(KT::LET) => self.parse_let()?,
                TT::KEYWORD(KT::WHILE) => self.parse_while()?,
                TT::KEYWORD(KT::RENDER) => self.parse_render()?,
                TT::KEYWORD(KT::RETURN) => self.parse_return()?,
                _ => self.parse_expr_to_stat()?,
            };
            Some(stat)
        } else {
            None
        };
        Ok(next)
    }

    fn parse_if(&mut self) -> Result<Statement, Error> {
        self.eat(TT::KEYWORD(KT::IF))?;
        let expr: Node = self.parse_expression()?;
        let body: Vec<Statement> = self.parse_block()?;
        let alter: Option<Box<Statement>> = if let Some(tk) = self.token.last() {
            match tk.ttype {
                TT::KEYWORD(KT::ELIF) => Some(Box::new(self.parse_elif()?)),
                TT::KEYWORD(KT::ELSE) => Some(Box::new(self.parse_else()?)),
                _ => None
            }
        } else {
            None
        };
        Statement::new(KT::IF, expr,  body, alter)
    }

    fn parse_elif(&mut self) -> Result<Statement, Error> {
        self.eat(TT::KEYWORD(KT::ELIF))?;
        let expr: Node = self.parse_expression()?;
        let body: Vec<Statement> = self.parse_block()?;
        let alter: Option<Box<Statement>> = if let Some(tk) = self.token.last() {
            match tk.ttype {
                TT::KEYWORD(KT::ELIF) => Some(Box::new(self.parse_elif()?)),
                TT::KEYWORD(KT::ELSE) => Some(Box::new(self.parse_else()?)),
                _ => None
            }
        } else {
            None
        };
        Statement::new(KT::IF, expr,  body, alter)
    }

    fn parse_else(&mut self) -> Result<Statement, Error> {
        self.eat(TT::KEYWORD(KT::ELSE))?;
        let expr: Node = Node::always();
        let body: Vec<Statement> = self.parse_block()?;
        Statement::new(KT::ELSE, expr, body, None)
    }

    fn parse_let(&mut self) -> Result<Statement, Error> {
        self.eat(TT::KEYWORD(KT::LET))?;
        let expr: Node = self.parse_assignment_target()?;
        let body: Vec<Statement> = if let Some(tk) = self.token.last() {
            if tk.ttype == TT::LBRACE {
                self.parse_block()?
            } else {
                let mut stat: Statement = self.parse_expr_to_stat()?;
                stat.ktype = KT::RETURN;
                vec![stat]
            }
        } else {
            return Error::stat_no_more_token();
        };
        Statement::new(KT::LET, expr, body, None)
    }

    fn parse_while(&mut self) -> Result<Statement, Error> {
        self.eat(TT::KEYWORD(KT::WHILE))?;
        let expr: Node = self.parse_expression()?;
        let body: Vec<Statement> = self.parse_block()?;
        Statement::new(KT::WHILE, expr, body, None)
    }

    fn parse_render(&mut self) -> Result<Statement, Error> {
        self.eat(TT::KEYWORD(KT::RENDER))?;
        let expr: Node = self.parse_expression()?;
        self.eat(TT::SEMICOL)?;
        Statement::new(KT::RENDER, expr, Vec::new(), None)
    }

    fn parse_return(&mut self) -> Result<Statement, Error> {
        self.eat(TT::KEYWORD(KT::RETURN))?;
        let expr: Node = self.parse_expression()?;
        self.eat(TT::SEMICOL)?;
        Statement::new(KT::RETURN, expr, Vec::new(), None)
    }

    fn parse_expr_to_stat(&mut self) -> Result<Statement, Error> {
        let expr: Node = self.parse_expression()?;
        self.eat(TT::SEMICOL)?;
        Statement::new(KT::NULL, expr, Vec::new(), None)
    }
}


impl Lexer<'_> {
    fn parse_block(&mut self) -> Result<Vec<Statement>, Error> {
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

    fn parse_assignment_target(&mut self) -> Result<Node, Error> {
        let mut ident: Node = if let Some(tk) = self.token.pop() {
            if tk.ttype == TT::NODE(NT::VALUE(VT::IDENT)) {
                Node::from(tk)?
            } else {
                return Error::expect_ident(tk.value)
            }
        } else {
            return Error::node_no_more_token()
        };

        if let Some(tk) = self.token.pop() {
            if tk.ttype != TT::NODE(NT::BINOPTR) || tk.value != "=" {
                return Error::expect_assignment(tk.value)
            }
        } else {
            return Error::node_no_more_token()
        }

        if let Some(tk) = self.token.last() {
            if tk.ttype == TT::PIPE {
                ident = self.cvt_ident_to_callable(ident)?;
            }
        }

        Ok(ident)
    }

    fn cvt_ident_to_callable(&mut self, mut ident: Node) -> Result<Node, Error> {
        ident.ntype = NT::CALLABLE;
        self.eat(TT::PIPE)?;

        loop {
            let next: Node = self.get_ident()?;
            ident.nodes.push(next);

            if let Some(s) = self.token.pop() {
                match s.ttype {
                    TT::COMMA => (),
                    TT::PIPE => return Ok(ident),
                    _ => return Error::invalid_args(s.value)
                }
            }
        };
    }

    fn get_ident(&mut self) -> Result<Node, Error> {
        if let Some(tk) = self.token.pop() {
            if tk.ttype == TT::NODE(NT::VALUE(VT::IDENT)) {
                Node::from(tk)
            } else {
                return Error::expect_ident(tk.value)
            }
        } else {
            return Error::node_no_more_token()
        }
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
    fn invalid_args(s: String) -> Result<Node, Error> {
        Err(Self { msg: format!("expect `|` or `,`, but see `{}`", s)})
    }

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