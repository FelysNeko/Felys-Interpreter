mod shared;
mod frontend;

use frontend::parse;

fn main() {
    let input: String = std::fs::read_to_string("playground.ely")
        .expect("cannot not open file");

    let _ = parse(input);
}
