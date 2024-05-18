use crate::shared::{
    ValueType as VT,
    Program,
    Environ,
    Output,
    Value,
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
        let mut ctr: usize = 0;

        for stat in self.body.iter() {
            if let Some(result) = stat.run(&mut env, &mut out, &mut ctr)? {
                out.body.push(format!("<{}>", result.value));
                return Ok(out);
            }
        }

        Ok(out)
    }
}


impl Output {
    pub fn render(&self) -> String {
        return self.body.join("\n")
    }
}
