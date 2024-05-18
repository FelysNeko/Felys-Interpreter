mod frontend;
mod backend;
mod shared;

use frontend::parse;
use shared::{
    Program,
    Output
};

const LIMIT: usize = 1000;

pub fn exec(code: String) -> String {
    let program: Program = match parse(code) {
        Ok(p) => p,
        Err(e) => return e.msg
    };
    
    let output: Output = match program.run() {
        Ok(o) => o,
        Err(e) => return e.msg
    };

    output.render()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = exec("let x = 'hello world'; render x;".to_string());
        assert_eq!(result, "hello world".to_string());
    }
}
