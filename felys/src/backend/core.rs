use std::collections::HashMap;

use crate::shared::{
    ValueType as VT,
    Program,
    Environ,
    Output,
    Value,
    Scope,
    Error
};


impl Program {
    pub fn run(&self) -> Result<Output, Error> {
        let elysia: Vec<(String, Value)> = vec![
            ("elysia".to_string(), Value { vtype: VT::STRING, value: "爱莉希雅".to_string() }),
            ("ELYSIA".to_string(), Value { vtype: VT::STRING, value: "「真我·人之律者」".to_string() })
        ];
        let mut env: Environ = Environ::new(elysia);
        let mut out: Output = Output::new();

        for stat in self.body.iter() {
            if let Some(result) = stat.run(&mut env, &mut out)? {
                out.lines.push(format!("<{}>", result.value));
                return Ok(out);
            }
        }

        Ok(out)
    }
}

impl Output {
    fn new() -> Self {
        Self { lines: Vec::new() }
    }

    pub fn render(&self) -> String {
        return self.lines.join("\n")
    }
}


impl Scope {
    pub(super) fn new(args: Vec<(String, Value)>) -> Self {
        let mut variable: HashMap<String, Value> = HashMap::new();
        for (k, v) in args {
            variable.insert(k, v);
        }
        Self {
            variable,
            callable: HashMap::new()
        }
    }
}


impl Environ {
    pub(super) fn new(args: Vec<(String, Value)>) -> Self {
        Self { body: vec![Scope::new(args)] }
    }
}