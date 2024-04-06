mod frontend;
mod runtime;
use frontend::Node;
use frontend::Lexer;


fn main() {
    let expr: String = String::from("z=1");
    let mut lexer: Lexer = Lexer::scan(expr);
    let mut entry: Node = lexer.parse();
    let result = entry.eval();
    println!("{:#?}", result);
}
