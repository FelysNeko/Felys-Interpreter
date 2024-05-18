use clap::Parser;
use std::fs::read_to_string;
use std::path::PathBuf;


#[derive(Parser)]
#[command(name = "felys")]
struct Cli {
    path: PathBuf,
}


fn main() {
    let args: Cli = Cli::parse();
    let code: String = read_to_string(&args.path).expect("no such file");
    let result: String = felys::exec(code);
    println!("{result}");
}
