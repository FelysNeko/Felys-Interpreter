mod frontend;

use frontend::Node;
use frontend::Lexer;


fn main() {
    let expr: String = String::from("!1");
    let entry: Node = Lexer::parse(expr);
    println!("{:#?}", entry);
}
