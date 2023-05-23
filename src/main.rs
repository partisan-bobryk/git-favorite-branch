use clap::Parser;
use gfb::{
    cli_definitions::{Cli, Commands},
    command_manager::CommandManager,
    config::Config,
    get_default_path,
    git_helpers::get_current_branch,
};
use std::collections::HashMap;

fn main() {
    let default_path = get_default_path();
    let mut config = Config::new(&default_path, HashMap::new());
    config.load();

    let mut cmd_manager = CommandManager { config };
    let cli = Cli::parse();

    match &cli.command {
        Commands::Add { key, branch_name } => {
            let branch = match branch_name {
                Some(v) => v.trim().to_owned(),
                None => get_current_branch().to_owned(),
            };

            cmd_manager.add_branch(key.trim().to_owned(), branch)
        }
        Commands::Use { key } => cmd_manager.switch_to_branch(key.trim().to_owned()),
        Commands::Del { key } => cmd_manager.delete_branch(key.trim().to_owned()),
        Commands::DelAll => cmd_manager.clear_branches(),
        Commands::Branch { key } => match key {
            Some(v) => cmd_manager.print_branch_name(v.trim().to_owned()),
            None => print!("{}", get_current_branch()),
        },
        Commands::New { key } => cmd_manager.create_new_branch(key.trim().to_owned()),
        Commands::Ls => cmd_manager.list_branches(),
    }
}
