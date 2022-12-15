mod cmd;

fn main() {
    let tag = cmd::increment_version(cmd::SemanticVersion::Major);
    println!("{}", tag);
}
