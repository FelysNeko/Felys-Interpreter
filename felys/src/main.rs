mod core;
mod environ;
mod frontend;
mod runtime;

use std::fs;

use core::frontend::Lexer;
use core::runtime::Output;
use core::{
    Program,
    Error
};
use std::process::exit;


fn main() {
    let file: String = fs::read_to_string("playground.ely").expect("failed to read");

    let main:Program = match Lexer::parse(file) {
        Ok(p) => p,
        Err(e) => {
            println!("{:?}", e);
            exit(1);
        }
    };
    
    let result: Result<Output, Error> = main.run();
    if let Some(e) = result.err() {
        println!("{:?}", e);
    }
}
