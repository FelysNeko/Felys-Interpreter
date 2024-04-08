mod frontend;
mod runtime;
mod error;

use frontend::Node;
use frontend::Lexer;
use runtime::Value;
use runtime::Scope;

use std::io::{self, Write};
use std::process::exit;

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
            Some(e) => e,
            None => exit(1)
        };
        let result: Value = entry.eval(&mut env);
        println!("{}", result.value);
    }
}
