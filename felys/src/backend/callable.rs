use crate::shared::{
    ValueType as VT,
    Environ,
    Callable,
    Output,
    Error,
    Value
};


impl Callable {
    pub(super) fn call(&self, args:Vec<Value>, out: &mut Output) -> Result<Value, Error> {
        if self.param.len() != args.len() {
            return Error::missing_parameter();
        }

        let args: Vec<(String, Value)> = self.param.clone().into_iter()
            .zip(args.into_iter())
            .collect();

        let mut env: Environ = Environ::new(args);
        
        for stat in &self.body {
            if let Some(result) = stat.run(&mut env, out)? {
                return Ok(result);
            };
        }

        Ok(Value::none())
    }
}


impl Value {
    fn none() -> Self {
        Self { vtype: VT::NONE, value: "none".to_string() }
    }
}


impl Error {
    fn missing_parameter() -> Result<Value, Error> {
        Err(Self { msg: format!("function missing parameter")})
    }
}
