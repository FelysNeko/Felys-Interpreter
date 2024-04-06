mod frontend;
mod runtime;
use frontend::core::Node;
use crate::frontend::core::Lexer;


fn main() {
    let expr: String = String::from("z=-4-(1-(3)-2)");
    let mut lexer: Lexer = Lexer::scan(expr);
    let entry: Node = lexer.parse();
    println!("{:#?}", entry);
}
