use colored::Colorize;

pub enum Handler<T, ES> {
    Good(T),
    Bad(ES)
}

pub struct Error;

pub struct ErrorStack {
    pub msg: Vec<String>
}


impl Error {
    pub fn unknown() -> String {
        String::from("error message not implented")
    }

    pub fn lexer_invalid_char(input:&String, i:usize) -> String {
        let mut space = String::new();
        for _ in 0..i {
            space.push(' ');
        }
        format!("{}\n{}{}\nError: invalid character", input, space, "^".red().bold())
    }
}

impl ErrorStack {
    pub fn new(e:String) -> Self {
        Self { msg:vec![e] }
    }

    pub fn push(mut self, new:String) -> Self {
        self.msg.push(new);
        self
    }
}

