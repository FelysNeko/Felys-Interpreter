mod core;
mod environ;
mod frontend;
mod runtime;
mod error;

use std::fs;

use core::frontend::Lexer;
use core::runtime::Output;
use core::{
    Program,
    Error
};


fn main() {
    let file: String = fs::read_to_string("playground.ely").expect("failed to read");
    let main: Program = Lexer::parse(file);
    let result: Result<Output, Error> = main.run();
    if let Some(e) = result.err() {
        println!("{:?}", e);
    }
}
