mod backend;
mod frontend;
mod shared;

pub fn exec(code: String) -> String {
    code
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
