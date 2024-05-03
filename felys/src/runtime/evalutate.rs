use crate::core::Program;
use crate::core::runtime::Scope;
use crate::core::frontend::Statement;

impl Program {
    pub fn run(&self) {
        let global: Scope = Scope::new(None);
        for stat in self.body.iter() {
            stat.eval();
        }
    }
}


impl Statement {
    fn eval(&self) {

    }
}