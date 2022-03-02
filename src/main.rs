use std::{collections::HashMap, env, path::Path};

use gfb::{args::Args, command_manager::CommandManager, config::Config};

fn get_default_path() -> String {
    let ref home_dir = env::var("HOME").unwrap_or("".to_string());
    let file_name = String::from(".git-favorite-branch-config");
    let path_raw: String;
    let default_path = match home_dir.len().cmp(&0) {
        std::cmp::Ordering::Greater => {
            path_raw = format!("{}/{}", home_dir, file_name).to_string();
            Path::new(&path_raw)
        }
        _ => {
            path_raw = format!("./{}", file_name).to_string();
            Path::new(&path_raw)
        }
    };
    String::from(default_path.to_str().unwrap())
}

fn main() {
    let default_path = get_default_path();
    let mut config = Config::new(&default_path, HashMap::new());
    config.load();

    let mut cmd_manager = CommandManager { config };
    let matches = Args::parse();

    match matches.subcommand() {
        Some(("add", sub_matches)) => {
            let key = sub_matches
                .value_of("SHORTCUT_KEY")
                .expect("required")
                .trim()
                .to_string();
            let branch = sub_matches
                .value_of("BRANCH_NAME")
                .expect("default branch required")
                .trim()
                .to_string();
            cmd_manager.add_branch(key, branch)
        }
        Some(("use", sub_matches)) => {
            let key = sub_matches
                .value_of("SHORTCUT_KEY")
                .expect("required")
                .trim()
                .to_string();
            cmd_manager.switch_to_branch(key)
        }
        Some(("del", sub_matches)) => {
            let key = sub_matches
                .value_of("SHORTCUT_KEY")
                .expect("required")
                .trim()
                .to_string();
            cmd_manager.delete_branch(key)
        }
        Some(("ls", _)) => cmd_manager.list_branches(),
        Some(("clr", _)) => cmd_manager.clear_branches(),
        Some(("prnt", sub_matches)) => {
            let key = sub_matches
                .value_of("SHORTCUT_KEY")
                .expect("required")
                .trim()
                .to_string();

            cmd_manager.print_branch_name(key)
        }
        _ => println!("Command doesn't exist"),
    }
}
