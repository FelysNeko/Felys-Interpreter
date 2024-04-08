mod frontend;
mod runtime;
mod error;

use frontend::Node;
use frontend::Lexer;
use runtime::Value;
use runtime::Scope;
use crate::error::Handler::{Good, Bad};

use std::io::{self, Write};

fn main() {
    let mut env: Scope = Scope::new();
    print!("\x1B[2J\x1B[1;1H");
    println!("Felys 0.1.0");
    loop {
        print!("> ");
        let mut expr: String = String::new();
        io::stdout().flush().expect("error");
        io::stdin().read_line(&mut expr).expect("error");
        expr.pop();

        let entry: Node = match Lexer::parse(expr) {
            Good(n) => n,
            Bad(b) => {
                println!("{}", b);
                continue;
            }
        };

        let result: Value = match entry.eval(&mut env) {
            Good(n) => n,
            Bad(b) => {
                println!("{}", b);
                continue;
            }
        };
        
        println!("{}", result.value);
    }
}
