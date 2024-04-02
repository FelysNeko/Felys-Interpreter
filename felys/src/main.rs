mod frontend;
mod runtime;
use crate::frontend::core::Lexer;


fn main() {
    let expr: String = String::from("x = 1 + 1");
    let lexer: Lexer = Lexer::scan(expr);
    
    for each in lexer.data {
        println!("{}", each.value);
    }
}
