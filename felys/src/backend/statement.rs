use crate::shared::{
    KeywordType as KT,
    Statement,
    Environ,
    Output,
    Error,
    Value
};


impl Statement {
    pub(super) fn run(&self, env: &mut Environ, out: &mut Output) -> Result<Option<Value>, Error> {
        match self.ktype {
            KT::IF => self.run_if(env, out),
            KT::ELIF => self.run_elif(env, out),
            KT::ELSE => self.run_else(env, out),
            KT::NULL => self.run_expression(env, out),
            KT::RENDER => self.run_render(env, out),
            KT::RETURN => self.run_return(env, out),
            KT::LET => self.run_let(env, out),
            KT::WHILE => self.run_while(env, out)
        }
    }

    fn run_let(&self, env: &mut Environ, out: &mut Output) -> Result<Option<Value>, Error> {
        todo!()
    }
    
    fn run_while(&self, env: &mut Environ, out: &mut Output) -> Result<Option<Value>, Error> {
        while self.expr.eval(env, out)?.parse_to_bool()? {
            if let Some(result) = self.run_block(env, out)? {
                return Ok(Some(result));
            }
        }
        Ok(None)
    }

    fn run_if(&self, env: &mut Environ, out: &mut Output) -> Result<Option<Value>, Error> {
        if self.expr.eval(env, out)?.parse_to_bool()? {
            self.run_block(env, out)
        } else if let Some(stat) = &self.alter {
            stat.run(env, out)
        } else {
            Ok(None)
        }
    }

    fn run_elif(&self, env: &mut Environ, out: &mut Output) -> Result<Option<Value>, Error> {
        if self.expr.eval(env, out)?.parse_to_bool()? {
            self.run_block(env, out)
        } else if let Some(stat) = &self.alter {
            stat.run(env, out)
        } else {
            Ok(None)
        }
    }

    fn run_else(&self, env: &mut Environ, out: &mut Output) -> Result<Option<Value>, Error> {
        self.run_block(env, out)
    }

    fn run_expression(&self, env: &mut Environ, out: &mut Output) -> Result<Option<Value>, Error> {
        let _ = self.expr.eval(env, out);
        Ok(None)
    }

    fn run_render(&self, env: &mut Environ, out: &mut Output) -> Result<Option<Value>, Error> {
        let result: Value = self.expr.eval(env, out)?;
        out.lines.push(result.value);
        Ok(None)
    }

    fn run_return(&self, env: &mut Environ, out: &mut Output) -> Result<Option<Value>, Error> {
        let result: Value = self.expr.eval(env, out)?;
        Ok(Some(result))
    }

    fn run_block(&self, env: &mut Environ, out: &mut Output) -> Result<Option<Value>, Error> {
        env.extend();
        for stat in self.body.iter() {
            if let Some(val) = stat.run(env, out)? {
                return Ok(Some(val));
            }
        }
        env.shrink();
        Ok(None)
    }
}