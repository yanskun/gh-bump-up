use std::process::{Command, Stdio};
use std::str;

#[allow(dead_code)]
pub enum SemanticVersion {
    Major,
    Minor,
    Patch,
}

pub fn get_latest_tag() -> String {
    let gh_release_first = Command::new("gh")
        .arg("release")
        .arg("list")
        .arg("--limit=1")
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

pub fn bump_version(semantic: SemanticVersion) -> String {
    let tag = get_latest_tag();
    let version: Vec<&str> = tag.split(".").collect();
    assert_eq!(version.len(), 3);

    let is_prefix = matches!(tag.chars().next(), Some('v'));

    let mut major = match is_prefix {
        true => version[0].trim_start_matches('v').parse::<u32>().unwrap(),
        false => version[0].parse::<u32>().unwrap(),
    };
    let mut minor = version[1].parse::<i32>().unwrap();
    let mut patch = version[2].parse::<i32>().unwrap();

    match semantic {
        SemanticVersion::Major => {
            major += 1;
            minor = 0;
            patch = 0;
        }
        SemanticVersion::Minor => {
            minor += 1;
            patch = 0;
        }
        SemanticVersion::Patch => {
            patch += 1;
        }
    }

    let new_version = format!("{}.{}.{}", major, minor, patch);
    match is_prefix {
        true => format!("v{}", new_version),
        false => new_version,
    }
}
