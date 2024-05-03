use crate::core::frontend::{
    TokenType as TT,
    Statement,
    Lexer,
    Node
};


impl Lexer<'_> {
    pub(super) fn parse_next(&mut self) -> Option<Statement> {
        if let Some(tk) = self.tokens.last() {
            let stat: Statement = match tk.kind {
                TT::WHILE => self._parse_while(),
                TT::PRINT => self._parse_print(),
                TT::IF => self._parse_if(),
                _ => self._parse_expr_to_stat(),
            };
            Some(stat)
        } else {
            None
        }
    }

    fn _parse_print(&mut self) -> Statement {
        self._must_eat(TT::PRINT);
        let mut stat: Statement = self._parse_expr_to_stat();
        stat.keyword = TT::PRINT;
        stat
    }

    fn _parse_while(&mut self) -> Statement {
        self._must_eat(TT::WHILE);
        let expr: Node = self._parse_expression();
        let body: Vec<Statement> = self._parse_block();

        Statement::new(
            TT::WHILE,
            expr,
            None,
            body
        )
    }

    fn _parse_if(&mut self) -> Statement {
        self._must_eat(TT::IF);
        let expr: Node = self._parse_expression();
        let body: Vec<Statement> = self._parse_block();
        let alter: Option<Box<Statement>> = if let Some(tk) = self.tokens.last() {
            match tk.kind {
                TT::ELIF => Some(Box::new(self._parse_elif())),
                TT::ELSE => Some(Box::new(self._parse_else())),
                _ => None
            }
        } else {
            None
        };
        
        Statement::new(
            TT::IF,
            expr,
            alter,
            body
        )
    }

    fn _parse_elif(&mut self) -> Statement {
        self._must_eat(TT::ELIF);
        let expr: Node = self._parse_expression();
        let body: Vec<Statement> = self._parse_block();
        let alter: Option<Box<Statement>> = if let Some(tk) = self.tokens.last() {
            match tk.kind {
                TT::ELIF => Some(Box::new(self._parse_elif())),
                TT::ELSE => Some(Box::new(self._parse_else())),
                _ => None
            }
        } else {
            None
        };

        Statement::new(
            TT::ELIF,
            expr,
            alter,
            body
        )
    }

    fn _parse_else(&mut self) -> Statement {
        self._must_eat(TT::ELSE);
        let body: Vec<Statement> = self._parse_block();

        Statement::new(
            TT::ELSE,
            Node::null(),
            None,
            body
        )
    }

    fn _parse_expr_to_stat(&mut self) -> Statement {
        let expr: Node = self._parse_expression();
        self._must_eat(TT::SEMICOL);

        Statement::new(
            TT::NULL,
            expr,
            None,
            Vec::new()
        )
    }

    fn _parse_block(&mut self) -> Vec<Statement> {
        self._must_eat(TT::LBRACE);
        let mut block: Vec<Statement> = Vec::new();

        while let Some(stat) = self.parse_next() {
            block.push(stat);

            if let Some(tk) = self.tokens.last() {
                if tk.kind == TT::RBRACE {
                    break;
                }
            }
        }
        
        self._must_eat(TT::RBRACE);
        block
    }
}


impl Statement {
    pub fn new(
        keyword: TT,
        expr: Node,
        alter: Option<Box<Statement>>,
        body: Vec<Statement>
    ) -> Self {
        Self { keyword, expr, alter, body }
    }
}