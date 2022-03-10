use std::process::{Command, ExitStatus};

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
