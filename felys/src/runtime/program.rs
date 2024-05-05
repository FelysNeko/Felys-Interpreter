use crate::core::{
    Program,
    Error
};
use crate::core::runtime::{
    Output,
    Scope
};

impl Program {
    pub fn run(self) -> Result<Output, Error> {
        let mut global: Scope = Scope::init();
        let mut output: Output = Output::new();
        for mut stat in self.body {
            stat.run(&mut global, &mut output)?;
        }
        Ok(output)
    }
}


impl Output {
    pub fn new() -> Self {
        Self { body: Vec::new() }
    }

    pub fn push(&mut self, value: String) {
        self.body.push(value)
    }

    pub fn render(&self) -> String {
        return self.body.join("\n")
    }
}


impl Error {
    pub fn render(&self) -> String {
        return self.msg.clone()
    }
}