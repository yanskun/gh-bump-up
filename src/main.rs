mod cmd;
use std::env;

fn main() {
    let arg = env::args().nth(1).unwrap();
    let semantic = match arg.as_str() {
        "--major" | "-M" => cmd::SemanticVersion::Major,
        "--minor" | "-m" => cmd::SemanticVersion::Minor,
        "--patch" | "-p" => cmd::SemanticVersion::Patch,
        _ => panic!("Invalid argument"),
    };

    let tag = cmd::increment_version(semantic);
    println!("{}", tag);
}
