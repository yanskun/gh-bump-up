mod cmd;
use colored::*;
use std::env;

fn main() {
    let arg = env::args().nth(1).unwrap();
    let semantic = match arg.as_str() {
        "--major" | "-M" => cmd::SemanticVersion::Major,
        "--minor" | "-m" => cmd::SemanticVersion::Minor,
        "--patch" | "-p" => cmd::SemanticVersion::Patch,
        "--help" | "-h" => {
            println!("\nMake GitHub Release Tag");
            println!("\n{}", format!("USAGE").bold());
            println!("  gh release-up [OPTIONS]");
            println!("\n{}", format!("OPTIONS").bold());
            println!("  -M, --major  Bump major version");
            println!("  -m, --minor  Bump minor version");
            println!("  -p, --patch  Bump patch version");
            println!("  -h, --help   Show help");
            println!("\n{}", format!("EXAMPLES").bold());
            println!("  $ gh release-up -M");
            println!("  $ gh release-up -m");
            println!("  $ gh release-up -p");
            return;
        }
        _ => panic!("Invalid argument"),
    };

    let tag = cmd::increment_version(semantic);
    println!("{}", tag);
}
