mod frontend;

use std::fs;

use frontend::Lexer;


fn main() {
    let file: String = fs::read_to_string("playground.ely").expect("failed to read");
    let entry = Lexer::parse(file);
    println!("{:#?}", entry);
}
