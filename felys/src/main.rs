mod core;
mod environ;
mod frontend;
mod runtime;

use std::fs;

use core::frontend::Lexer;
use core::runtime::Output;
use core::Program;
use std::process::exit;


fn main() {
    let file: String = fs::read_to_string("playground.ely").expect("failed to read");

    let main: Program = match Lexer::parse(file) {
        Ok(p) => p,
        Err(e) => {
            println!("SyntaxError: {}", e.render());
            exit(1);
        }
    };

    let out: Output = match main.run() {
        Ok(r) => r,
        Err(e) => {
            println!("RuntimeError: {}", e.render());
            exit(1);
        }
    };

    let s: String = out.render();
    println!("{}", s);
}
