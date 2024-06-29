use crate::shared::program::Program;

mod backend;
mod frontend;
mod shared;


pub fn exec(code: String) -> String {
    let main = match Program::init(code) {
        Ok(m) => m,
        Err(e) => return e.body
    };
    
    String::from("pass")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = exec("x = |a|a;".to_string());
        assert_eq!(result, "pass");
    }
}
