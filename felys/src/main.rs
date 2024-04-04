mod frontend;
mod runtime;
use crate::frontend::core::Lexer;


fn main() {
    let expr: String = String::from("2x12s");
    let lexer: Lexer = Lexer::scan(expr);
    
    for each in lexer.data.iter().rev() {
        println!("{}", each.value);
    }
}
