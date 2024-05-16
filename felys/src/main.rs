mod shared;
mod frontend;
mod backend;

use std::process::exit;

use frontend::parse;
use shared::{
    Program,
    Output
};


fn main() {
    let input: String = std::fs::read_to_string("playground.ely")
        .expect("cannot not open file");

    let program: Program = match parse(input) {
        Ok(p) => p,
        Err(e) => {
            println!("{}", e.msg);
            exit(1)
        }
    };
    
    let output: Output = match program.run() {
        Ok(o) => o,
        Err(e) => {
            println!("{}", e.msg);
            exit(1)
        }
    };

    println!("{}", output.render());
}
