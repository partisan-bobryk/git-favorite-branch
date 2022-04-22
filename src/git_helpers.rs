use std::{
    io::Error,
    process::{Command, ExitStatus, Output},
};

use serde_json::Value;

pub fn get_current_branch() -> String {
    let branch_name_raw = Command::new("sh")
        .arg("-c")
        .arg("git branch --show-current")
        .output()
        .expect("Failed to grab branch name");
    String::from_utf8(branch_name_raw.stdout).unwrap()
}

pub fn switch_branch(branch_name: &str) -> Result<ExitStatus, std::io::Error> {
    Command::new("sh")
        .arg("-c")
        .arg(format!("git checkout {:?}", branch_name))
        .status()
}

pub fn create_branch(branch_name: &str) -> Result<ExitStatus, std::io::Error> {
    Command::new("sh")
        .arg("-c")
        .arg(format!("git checkout -b {:?}", branch_name))
        .status()
}

pub fn get_local_hash() -> Result<Output, Error> {
    Command::new("sh")
        .arg("-c")
        .arg("git rev-parse --verify HEAD")
        .output()
}

pub fn get_latest_release_version() -> Option<String> {
    let latest_gh_release_raw = match Command::new("sh")
        .arg("-c")
        .arg("curl https://api.github.com/repos/veprUA/git-favorite-branch/releases/latest")
        .output()
    {
        Ok(val) => Some(val.stdout),
        Err(err) => {
            eprintln!("Failed to fetch latest release: {}", err);
            None
        }
    };

    if latest_gh_release_raw.is_none() {
        return None;
    }

    let parsed_gh_release: Option<Value> =
        match serde_json::from_slice(&latest_gh_release_raw.unwrap()) {
            Ok(val) => Some(val),
            Err(_) => {
                eprintln!("Failed to parse release data");
                None
            }
        };

    if parsed_gh_release.is_none() {
        return None;
    }

    Some(
        parsed_gh_release.unwrap()["tag_name"]
            .to_string()
            .replace("\"", ""),
    )
}
