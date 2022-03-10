use clap::{arg, App, AppSettings, ArgMatches};

use crate::git_helpers;

pub struct Args {}

impl Args {
    pub fn parse() -> ArgMatches {
        let default_branch_name = git_helpers::get_current_branch();
        let matches = App::new("gfb")
        .about("Quickly switch between branches")
        .setting(AppSettings::AllowExternalSubcommands | AppSettings::SubcommandRequiredElseHelp | AppSettings::AllowInvalidUtf8ForExternalSubcommands)
        .subcommand(
            App::new("add")
                .about("Add current branch to favorites")
                .arg(arg!(<SHORTCUT_KEY> "A short reference for a branch. Used when switching branches"))
                .arg(arg!([BRANCH_NAME] "Add branch other than the current one").default_value(default_branch_name.as_str()))
                .setting(AppSettings::ArgRequiredElseHelp),
        )
        .subcommand(App::new("use").about("Switch to a different branch").arg(arg!(<SHORTCUT_KEY> "Key used when saving the branch")).setting(AppSettings::ArgRequiredElseHelp))
        .subcommand(App::new("del").about("Delete favorited branch").arg(arg!(<SHORTCUT_KEY> "Key used when saving the branch")).setting(AppSettings::ArgRequiredElseHelp))
        .subcommand(App::new("prnt").about("Print a favorited branch").arg(arg!(<SHORTCUT_KEY> "Key used when saving the branch")).setting(AppSettings::ArgRequiredElseHelp))
        .subcommand(App::new("clr").about("Clear all saved favorites"))
        .subcommand(App::new("ls").about("Display all favorited branches and their keys"))
        .subcommand(App::new("install").about("Install provided version of gfb").arg(arg!(<VERSION> "Version number in a semver format. ie v0.1.0")).setting(AppSettings::ArgRequiredElseHelp))
        .subcommand(App::new("version").about("Display the current version of the binary"))
        .get_matches();

        matches
    }
}
