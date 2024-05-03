use std::process::exit;

use crate::core::frontend::{
    TokenType as TT,
    Statement
};
use crate::core::runtime::{
    Scope,
    Value
};

impl Statement {
    pub fn run(self, env: &Scope) {
        match self.keyword {
            TT::PRINT => self._run_print(env),
            TT::NULL => self._run_expression(env),
            _ => {
                println!("keyword [{:?}] not impl", self.keyword);
                exit(1)
            }
        }
    }

    fn _run_print(self, env: &Scope) {
        let result: Value = self.expr.eval(env);
        println!("{:?}", result);
    }

    fn _run_expression(self, env: &Scope) {
        self.expr.eval(env);
    }
}