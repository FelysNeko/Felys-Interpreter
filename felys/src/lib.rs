use crate::shared::Program;

mod backend;
mod frontend;
mod shared;


pub fn exec(code: String) -> String {
    let _ = Program::load(code);
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = exec("elysia;".to_string());
        assert_eq!(result, "elysia;");
    }
}
