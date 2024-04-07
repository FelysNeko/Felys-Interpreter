mod frontend;
mod runtime;
use frontend::Node;
use frontend::Lexer;
use runtime::Value;


fn main() {
    let expr: String = String::from("");
    let entry: Node = Lexer::parse(expr);
    let result: Value = entry.eval();
    println!("{:#?}", result);
}
