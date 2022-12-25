use colored::*;

pub enum SemanticVersion {
    Major,
    Minor,
    Patch,
}

pub fn get_semantic(s: &str) -> Option<SemanticVersion> {
    match s {
        "--major" | "-M" => Some(SemanticVersion::Major),
        "--minor" | "-m" => Some(SemanticVersion::Minor),
        "--patch" | "-p" => Some(SemanticVersion::Patch),
        "--help" | "-h" => {
            call_help();
            None
        }
        _ => {
            println!(
                "Error: {} is invalid argument",
                format!("'{}'", s).red().bold()
            );
            call_help();
            None
        }
    }
}

fn call_help() {
    println!("\n{}", format!("USAGE").bold());
    println!("  gh bump-up [OPTIONS]");
    println!("\n{}", format!("OPTIONS").bold());
    println!("  -M, --major  Bump major version");
    println!("  -m, --minor  Bump minor version");
    println!("  -p, --patch  Bump patch version");
    println!("  -h, --help   Show help");
    println!("\n{}", format!("EXAMPLES").bold());
    println!("  $ gh bump-up -M");
    println!("  $ gh bump-up -m");
    println!("  $ gh bump-up -p");
}
