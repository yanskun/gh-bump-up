mod option;
use std::num::ParseIntError;
use std::process::{Command, Stdio};
use std::str;

pub fn main(s: &str) -> String {
    let v = option::get_semantic(s);
    match v {
        Some(semantic) => {
            let tag = get_latest_tag();
            bump_version(tag, semantic).unwrap()
        }
        None => String::from(""),
    }
}

fn bump_version(tag: String, semantic: option::SemanticVersion) -> Result<String, ParseIntError> {
    let version: Vec<&str> = tag.split(".").collect();
    assert_eq!(version.len(), 3);

    let is_prefix = matches!(tag.chars().next(), Some('v'));

    let mut major = match is_prefix {
        true => version[0].trim_start_matches('v').parse::<u32>()?,
        false => version[0].parse::<u32>()?,
    };
    let mut minor = version[1].parse::<i32>()?;
    let mut patch = version[2].parse::<i32>()?;

    match semantic {
        option::SemanticVersion::Major => {
            major += 1;
            minor = 0;
            patch = 0;
        }
        option::SemanticVersion::Minor => {
            minor += 1;
            patch = 0;
        }
        option::SemanticVersion::Patch => {
            patch += 1;
        }
    }

    let new_version = format!("{}.{}.{}", major, minor, patch);
    Ok(match is_prefix {
        true => format!("v{}", new_version),
        false => new_version,
    })
}

#[test]
fn test_bump_version() {
    assert_eq!(
        bump_version(String::from("v1.2.3"), option::SemanticVersion::Major),
        Ok("v2.0.0".to_string())
    );
    assert_eq!(
        bump_version(String::from("v1.2.3"), option::SemanticVersion::Minor),
        Ok("v1.3.0".to_string())
    );
    assert_eq!(
        bump_version(String::from("v1.2.3"), option::SemanticVersion::Patch),
        Ok("v1.2.4".to_string())
    );
    assert_eq!(
        bump_version(String::from("1.2.3"), option::SemanticVersion::Major),
        Ok("2.0.0".to_string())
    );
    assert_eq!(
        bump_version(String::from("1.2.3"), option::SemanticVersion::Minor),
        Ok("1.3.0".to_string())
    );
    assert_eq!(
        bump_version(String::from("1.2.3"), option::SemanticVersion::Patch),
        Ok("1.2.4".to_string())
    );

    assert!(bump_version(String::from("v1.2.3-rc"), option::SemanticVersion::Major).is_err());
    // assert!(bump_version(String::from("v1.2"), option::SemanticVersion::Minor).is_err());
}

fn get_latest_tag() -> String {
    let gh_release_first = Command::new("gh")
        .arg("release")
        .arg("list")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let grep_latest = Command::new("grep")
        .arg("Latest")
        .stdin(gh_release_first.stdout.unwrap())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let awk_tag = Command::new("awk")
        .arg("{print $1}")
        .stdin(grep_latest.stdout.unwrap())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let output = awk_tag.wait_with_output().unwrap();
    let result = str::from_utf8(&output.stdout)
        .unwrap()
        .lines()
        .next()
        .unwrap();
    result.to_string()
}
