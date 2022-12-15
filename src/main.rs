mod cmd;
use std::env;

fn main() {
    let arg = env::args().nth(1).unwrap();
    let semantic = match arg.as_str() {
        "--major" => cmd::SemanticVersion::Major,
        "--minor" => cmd::SemanticVersion::Minor,
        "--patch" => cmd::SemanticVersion::Patch,
        _ => panic!("Invalid argument"),
    };

    let tag = cmd::increment_version(semantic);
    println!("{}", tag);
}
