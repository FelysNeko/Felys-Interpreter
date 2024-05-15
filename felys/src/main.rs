mod shared;
mod frontend;
mod backend;

use frontend::parse;


fn main() {
    let input: String = std::fs::read_to_string("playground.ely")
        .expect("cannot not open file");

    match parse(input) {
        Ok(p) => println!("{:#?}", p),
        Err(e) => println!("{}", e.msg)
    };
}
