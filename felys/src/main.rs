mod core;
mod frontend;

use std::fs;

use core::Lexer;
use core::Program;


fn main() {
    let file: String = fs::read_to_string("playground.ely").expect("failed to read");
    let entry: Program = Lexer::parse(file);
    println!("{:#?}", entry);
}
