mod cmd;
use std::env;

fn main() {
    let arg = env::args().nth(1).unwrap();
    let tag = cmd::main(&arg);
    println!("{}", tag);
}
