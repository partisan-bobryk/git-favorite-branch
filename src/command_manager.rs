use crate::{
    config::Config,
    git_helpers::{create_branch, switch_branch},
};
use std::{collections::HashMap, process::exit};

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
                eprintln!("No branch name set for key {}", key);
                exit(1)
            }
        };
    }

    pub fn delete_branch(&mut self, key: String) {
        self.config.state.remove(&key).unwrap();
        self.config.save()
    }

    pub fn create_new_branch(&self, key: String) {
        match self.config.state.get(&key) {
            Some(branch_name) => create_branch(branch_name).unwrap(),
            None => {
                eprintln!("No branch name set for key {}", key);
                exit(1)
            }
        };
    }

    pub fn list_branches(&self) {
        let ref state = self.config.state;
        let state_iter = state.into_iter();
        if state_iter.len().eq(&0) {
            eprintln!("No favorite branches saved")
        }

        state_iter.for_each(|(k, v)| println!("{} -> {}", k, v))
    }

    pub fn clear_branches(&mut self) {
        self.config.state = HashMap::new();
        self.config.save()
    }

    pub fn print_branch_name(&self, key: String) {
        let branch_name = self.config.state.get(&key).unwrap();
        print!("{}", branch_name);
    }
}
