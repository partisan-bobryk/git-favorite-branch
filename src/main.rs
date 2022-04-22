use gfb::{args::Args, command_manager::CommandManager, config::Config, get_default_path};
use std::{collections::HashMap, env, process};

fn main() {
    let default_path = get_default_path();
    let build_target = option_env!("BUILD_TARGET").unwrap_or("");
    let mut config = Config::new(&default_path, HashMap::new(), &build_target);
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

            cmd_manager.add_branch(key, branch);
        }
        Some(("use", sub_matches)) => {
            let key = sub_matches
                .value_of("SHORTCUT_KEY")
                .expect("required")
                .trim()
                .to_string();

            cmd_manager.switch_to_branch(key);
        }
        Some(("del", sub_matches)) => {
            let key = sub_matches
                .value_of("SHORTCUT_KEY")
                .expect("required")
                .trim()
                .to_string();

            cmd_manager.delete_branch(key);
        }
        Some(("new", sub_matches)) => {
            let key = sub_matches
                .value_of("SHORTCUT_KEY")
                .expect("required")
                .trim()
                .to_string();

            cmd_manager.create_new_branch(key);
        }
        Some(("ls", _)) => cmd_manager.list_branches(),
        Some(("clr", _)) => cmd_manager.clear_branches(),
        Some(("version", _)) => cmd_manager.get_app_version(),
        Some(("prnt", sub_matches)) => {
            let key = sub_matches
                .value_of("SHORTCUT_KEY")
                .expect("required")
                .trim()
                .to_string();

            cmd_manager.print_branch_name(key);
        }
        Some(("install", sub_matches)) => {
            let version = sub_matches.value_of("VERSION");

            cmd_manager.install_binary(version).unwrap();
        }
        _ => {
            eprintln!("Command does not exist");
            process::exit(1)
        }
    }
}
