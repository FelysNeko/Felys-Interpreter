use crate::core::Program;
use crate::core::runtime::Scope;

impl Program {
    pub fn run(self) {
        let mut global: Scope = Scope::init();
        for mut stat in self.body {
            stat.run(&mut global);
        }
    }
}