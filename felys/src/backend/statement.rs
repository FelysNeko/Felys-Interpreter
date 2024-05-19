use crate::shared::{
    KeywordType as KT,
    NodeType as NT,
    Statement,
    Callable,
    Environ,
    Output,
    Error,
    Value,
};
use crate::LIMIT;


impl Statement {
    pub(super) fn run(
        &self,
        env: &mut Environ,
        out: &mut Output,
        ctr: &mut usize,
    ) -> Result<Option<Value>, Error> {
        if *ctr < LIMIT {
            *ctr += 1 
        } else {
            return Error::exceed_limit()
        }

        match self.ktype {
            KT::IF => self.run_if(env, out, ctr),
            KT::ELIF => self.run_elif(env, out, ctr),
            KT::ELSE => self.run_else(env, out, ctr),
            KT::NULL => self.run_expression(env, out, ctr),
            KT::RENDER => self.run_render(env, out, ctr),
            KT::RETURN => self.run_return(env, out, ctr),
            KT::LET => self.run_let(env, out, ctr),
            KT::WHILE => self.run_while(env, out, ctr)
        }
    }

    fn run_let(
        &self,
        env: &mut Environ,
        out: &mut Output,
        ctr: &mut usize
    ) -> Result<Option<Value>, Error> {
        if self.expr.ntype == NT::CALLABLE {
            let args: Vec<String> = self.expr.nodes.iter().map(|n| n.value.clone()).collect();
            let func: Callable = Callable::new(args, self.body.clone());
            env.load(&self.expr.value, func)?;
        } else {
            if let Some(stat) = self.body.first() {
                let result: Value = stat.expr.eval(env, out, ctr)?;
                env.declare(&self.expr.value, result)?;
            }
        }
        Ok(None)
    }
    
    fn run_while(
        &self,
        env: &mut Environ,
        out: &mut Output,
        ctr: &mut usize
    ) -> Result<Option<Value>, Error> {
        while self.expr.eval(env, out, ctr)?.parse_to_bool()? {
            if let Some(result) = self.run_block(env, out, ctr)? {
                return Ok(Some(result));
            }
        }
        Ok(None)
    }

    fn run_if(
        &self,
        env: &mut Environ,
        out: &mut Output,
        ctr: &mut usize
    ) -> Result<Option<Value>, Error> {
        if self.expr.eval(env, out, ctr)?.parse_to_bool()? {
            self.run_block(env, out, ctr)
        } else if let Some(stat) = &self.alter {
            stat.run(env, out, ctr)
        } else {
            Ok(None)
        }
    }

    fn run_elif(
        &self,
        env: &mut Environ,
        out: &mut Output,
        ctr: &mut usize
    ) -> Result<Option<Value>, Error> {
        if self.expr.eval(env, out, ctr)?.parse_to_bool()? {
            self.run_block(env, out, ctr)
        } else if let Some(stat) = &self.alter {
            stat.run(env, out, ctr)
        } else {
            Ok(None)
        }
    }

    fn run_else(
        &self,
        env: &mut Environ,
        out: &mut Output,
        ctr: &mut usize
    ) -> Result<Option<Value>, Error> {
        self.run_block(env, out, ctr)
    }

    fn run_expression(
        &self,
        env: &mut Environ,
        out: &mut Output,
        ctr: &mut usize
    ) -> Result<Option<Value>, Error> {
        self.expr.eval(env, out, ctr)?;
        Ok(None)
    }

    fn run_render(
        &self,
        env: &mut Environ,
        out: &mut Output,
        ctr: &mut usize
    ) -> Result<Option<Value>, Error> {
        let result: Value = self.expr.eval(env, out, ctr)?;
        out.body.push(result.value);
        Ok(None)
    }

    fn run_return(
        &self,
        env: &mut Environ,
        out: &mut Output,
        ctr: &mut usize
    ) -> Result<Option<Value>, Error> {
        let result: Value = self.expr.eval(env, out, ctr)?;
        Ok(Some(result))
    }

    fn run_block(
        &self,
        env: &mut Environ,
        out: &mut Output,
        ctr: &mut usize
    ) -> Result<Option<Value>, Error> {
        env.extend();
        for stat in self.body.iter() {
            if let Some(val) = stat.run(env, out, ctr)? {
                return Ok(Some(val));
            }
        }
        env.shrink();
        Ok(None)
    }
}


impl Error {
    fn exceed_limit() -> Result<Option<Value>, Error> {
        Err(Self {
            msg: format!("exceed {} steps exec limits", LIMIT)
        })
    }
}