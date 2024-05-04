use std::process::exit;

use crate::core::Error;
use crate::core::frontend::{
    TokenType as TT,
    Statement
};
use crate::core::runtime::{
    Output,
    Scope,
    Value
};


macro_rules! test_condition {
    ($self:ident, $env:ident) => {
        match $self.expr.eval($env) {
            Ok(val) => match val._parse_to_bool() {
                Ok(test) => test,
                Err(e) => return Err(e)
            }
            Err(e) => return Err(e)
        }
    };
}


impl Statement {
    pub fn run(&mut self, env: &mut Scope, out: &mut Output) -> Result<(), Error> {
        match self.keyword {
            TT::PRINT => self._run_print(env, out),
            TT::WHILE => self._run_while(env, out),
            TT::IF => self._run_if(env, out),
            TT::ELIF => self._run_elif(env, out),
            TT::ELSE => self._run_else(env, out),
            TT::NULL => self._run_expression(env),
            _ => {
                println!("keyword [{:?}] not impl", self.keyword);
                exit(1)
            }
        }
    }

    fn _run_block(&mut self, env: &mut Scope, out: &mut Output) -> Result<(), Error> {
        env.extend();
        for stat in self.body.iter_mut() {
            stat.run(env, out)?;
        }
        env.shrink();
        Ok(())
    }

    fn _run_else(&mut self, env: &mut Scope, out: &mut Output) -> Result<(), Error> {
        self._run_block(env, out)
    }

    fn _run_elif(&mut self, env: &mut Scope, out: &mut Output) -> Result<(), Error> {
        if test_condition!(self, env) {
            self._run_block(env, out)?;
        } else if let Some(stat) = self.alter.as_mut() {
            stat.run(env, out)?;
        }
        Ok(())
    }

    fn _run_if(&mut self, env: &mut Scope, out: &mut Output) -> Result<(), Error> {
        if test_condition!(self, env) {
            self._run_block(env, out)?;
        } else if let Some(stat) = self.alter.as_mut() {
            stat.run(env, out)?;
        }
        Ok(())
    }

    fn _run_while(&mut self, env: &mut Scope, out: &mut Output) -> Result<(), Error> {
        while test_condition!(self, env) {
            self._run_block(env, out)?;
        }
        Ok(())
    }

    fn _run_print(&mut self, env: &mut Scope, out: &mut Output) -> Result<(), Error> {
        let result: Value = match self.expr.eval(env) {
            Ok(val) => val,
            Err(e) => return Err(e)
        };
        out.push(result.value);
        Ok(())
    }

    fn _run_expression(&mut self, env: &mut Scope) -> Result<(), Error> {
        match self.expr.eval(env) {
            Ok(_) => Ok(()),
            Err(e) => return Err(e)
        }
    }
}
