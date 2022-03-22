use crate::{
    config::Config,
    git_helpers::{get_latest_release_version, switch_branch},
};
use std::{
    collections::HashMap,
    env,
    io::{Error, ErrorKind},
    process::{exit, Command, ExitStatus},
};

pub struct CommandManager {
    pub config: Config,
}

impl CommandManager {
    pub fn add_branch(&mut self, key: String, branch_name: String) {
        self.config.state.insert(key, branch_name);
        self.config.save()
    }

    pub fn switch_to_branch(&self, key: String) {
        match self.config.state.get(&key) {
            Some(branch_name) => switch_branch(branch_name).unwrap(),
            None => {
                println!("No branch name set for key {}", key);
                exit(1)
            }
        };
    }

    pub fn delete_branch(&mut self, key: String) {
        self.config.state.remove(&key).unwrap();
        self.config.save()
    }

    pub fn list_branches(&self) {
        let ref state = self.config.state;
        state
            .into_iter()
            .for_each(|(k, v)| println!("{} -> {}", k, v))
    }

    pub fn clear_branches(&mut self) {
        self.config.state = HashMap::new();
        self.config.save()
    }

    pub fn print_branch_name(&self, key: String) {
        let branch_name = self.config.state.get(&key).unwrap();
        println!("{}", branch_name);
    }

    pub fn get_app_version(&self) {
        if env::var("GFB_NO_UPDATE_CHECK").is_ok() {
            println!("{}", self.config.version);
            return;
        }

        let latest_version = get_latest_release_version();

        if self.has_new_update(latest_version.clone()) {
            eprintln!("New version available! Run `gfb install`");
        }

        println!(
            "{} (Latest {})",
            self.config.version,
            latest_version.unwrap()
        );
    }

    pub fn install_binary(&self, version: Option<&str>) -> Result<ExitStatus, std::io::Error> {
        let latest_version = get_latest_release_version();
        if latest_version.is_none() && version.is_none() {
            eprintln!("Installation versionn is required");
            return Err(Error::new(
                ErrorKind::Other,
                "Installation versionn is required!",
            ));
        }
        let latest_version = latest_version.unwrap();

        let download_version = version.unwrap_or_else(|| &latest_version).trim();

        Command::new("sh")
        .arg("-c")
        .arg(format!(
            "curl https://raw.githubusercontent.com/VeprUA/git-favorite-branch/main/bin/install.sh | sh -s {:?} {:?}",
            download_version, self.config.target
        ))
        .status()
    }

    pub fn has_new_update(&self, new_version: Option<String>) -> bool {
        let current_version = &self.config.version;

        if new_version.is_none() {
            return false;
        }

        current_version.ne(&new_version.unwrap())
    }
}
