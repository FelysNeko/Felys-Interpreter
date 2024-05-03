use crate::core::Program;
use crate::core::runtime::Scope;

impl Program {
    pub fn run(self) {
        let global: Scope = Scope::new(None);
        for stat in self.body {
            stat.run(&global);
        }
    }
}