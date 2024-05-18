use crate::shared::{
    ValueType as VT,
    Callable,
    Environ,
    Output,
    Scope,
    Error,
    Value
};


impl Environ {
    pub(super) fn extend(&mut self) {
        self.body.push(Scope::new(Vec::new()));
    }

    pub(super) fn shrink(&mut self) {
        self.body.pop();
    }

    pub(super) fn declare(&mut self, k:&String, v:Value)  -> Result<(), Error> {
        if let Some(scope) = self.body.last_mut() {
            match scope.variable.insert(k.clone(), v) {
                Some(_) => Error::var_already_exist(k),
                None => Ok(())
            }
        } else {
            Error::no_more_scope()
        }
    }

    pub(super) fn assign(&mut self, k:&String, v:Value) -> Result<(), Error> {
        for scope in self.body.iter_mut().rev() {
            if scope.variable.contains_key(k) {
                scope.variable.insert(k.clone(), v);
                return Ok(());
            }
        }
        Error::assign_to_dne_var(k)
    }

    pub(super) fn get(&self, k:&String) -> Result<Value, Error> {
        for scope in self.body.iter().rev() {
            if let Some(v) = scope.variable.get(k) {
                return Ok(v.clone());
            }
        }
        Error::get_dne_var(k)
    }

    pub(super) fn load(&mut self, k:&String, v:Callable) -> Result<(), Error> {
        if let Some(scope) = self.body.last_mut() {
            match scope.callable.insert(k.clone(), v) {
                Some(_) => Error::func_already_exist(k),
                None => Ok(())
            }
        } else {
            Error::no_more_scope()
        }
    }

    pub(super) fn call(
        &self,
        k:&String,
        args:Vec<Value>,
        out: &mut Output,
        ctr: &mut usize
    ) -> Result<Value, Error> {
        for scope in self.body.iter().rev() {
            if let Some(c) = scope.callable.get(k) {
                return c.call(args, out, ctr);
            }
        }
        Error::call_dne_func(k)
    }
}


impl Callable {
    fn call(
        &self,
        args:Vec<Value>,
        out: &mut Output,
        ctr: &mut usize
    ) -> Result<Value, Error> {
        if self.args.len() != args.len() {
            return Error::missing_parameter();
        }

        let args: Vec<(String, Value)> = self.args.clone()
            .into_iter()
            .zip(args.into_iter())
            .collect();

        let mut env: Environ = Environ::new(args);
        
        for stat in &self.body {
            if let Some(result) = stat.run(&mut env, out, ctr)? {
                return Ok(result);
            };
        }

        Value::new(VT::NONE, "none".to_string())
    }
}


impl Error {
    fn no_more_scope() -> Result<(), Error> {
        Err(Self { msg: format!("no more scope")})
    }

    fn var_already_exist(s: &String) -> Result<(), Error> {
        Err(Self { msg: format!("variable `{}` already exist", s)})
    }

    fn assign_to_dne_var(s: &String) -> Result<(), Error> {
        Err(Self { msg: format!("cannot assign to undeclared variable `{}`", s)})
    }

    fn get_dne_var(s: &String) -> Result<Value, Error> {
        Err(Self { msg: format!("cannot get undeclared variable `{}`", s)})
    }

    fn func_already_exist(s: &String) -> Result<(), Error> {
        Err(Self { msg: format!("callable `{}` already exist", s)})
    }

    fn call_dne_func(s: &String) -> Result<Value, Error> {
        Err(Self { msg: format!("cannot call undeclared callable `{}`", s)})
    }

    fn missing_parameter() -> Result<Value, Error> {
        Err(Self { msg: format!("function missing parameter")})
    }
}
