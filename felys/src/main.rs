mod frontend;
mod runtime;
use crate::frontend::core::Lexer;


fn main() {
    let expr: String = String::from("x-2*('123'+3)");
    let lexer: Lexer = Lexer::scan(expr);
    
    for each in lexer.tokens.iter().rev() {
        println!("{:?}\t{}", each.kind, each.value);
    }
}
