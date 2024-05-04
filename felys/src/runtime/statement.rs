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
    pub fn run(&mut self, env: &mut Scope) {
        match self.keyword {
            TT::PRINT => self._run_print(env),
            TT::WHILE => self._run_while(env),
            TT::IF => self._run_if(env),
            TT::ELIF => self._run_elif(env),
            TT::ELSE => self._run_else(env),
            TT::NULL => self._run_expression(env),
            _ => {
                println!("keyword [{:?}] not impl", self.keyword);
                exit(1)
            }
        }
    }

    fn _run_block(&mut self, env: &mut Scope) {
        env.extend();
        for stat in self.body.iter_mut() {
            stat.run(env);
        }
        env.shrink();
    }

    fn _run_else(&mut self, env: &mut Scope) {
        self._run_block(env);
    }

    fn _run_elif(&mut self, env: &mut Scope) {
        if self.expr.eval(env)._parse_to_bool() {
            self._run_block(env);
        } else if let Some(stat) = self.alter.as_mut() {
            stat.run(env);
        }
    }

    fn _run_if(&mut self, env: &mut Scope) {
        if self.expr.eval(env)._parse_to_bool() {
            self._run_block(env);
        } else if let Some(stat) = self.alter.as_mut() {
            stat.run(env);
        }
    }

    fn _run_while(&mut self, env: &mut Scope) {
        while self.expr.eval(env)._parse_to_bool() {
            self._run_block(env);
        }
    }

    fn _run_print(&mut self, env: &mut Scope) {
        let result: Value = self.expr.eval(env);
        println!("{:?}", result.value);
    }

    fn _run_expression(&mut self, env: &mut Scope) {
        self.expr.eval(env);
    }
}