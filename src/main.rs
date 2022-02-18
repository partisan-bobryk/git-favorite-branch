use clap::{arg, App, AppSettings};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{BufReader, BufWriter},
    path::Path,
    process::{exit, Command, ExitStatus},
};

fn main() {
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
    let mut config = Config::new(default_path.to_str().unwrap());
    config.load();
    let matches = App::new("gfb")
        .about("Quickly switch between branches")
        .setting(AppSettings::AllowExternalSubcommands | AppSettings::SubcommandRequiredElseHelp | AppSettings::AllowInvalidUtf8ForExternalSubcommands)
        .subcommand(
            App::new("add")
                .about("Add current branch to favorites")
                .arg(arg!(<SHORTCUT_KEY> "A short reference for a branch. Used when switching branches"))
                .arg(arg!([BRANCH_NAME] "Add branch other than the current one").default_value(get_current_branch().as_str()))
                .setting(AppSettings::ArgRequiredElseHelp),
        )
        .subcommand(App::new("use").about("Switch to a different branch").arg(arg!(<SHORTCUT_KEY> "Key used when saving the branch")).setting(AppSettings::ArgRequiredElseHelp))
        .subcommand(App::new("del").about("Delete favorited branch").arg(arg!(<SHORTCUT_KEY> "Key used when saving the branch")).setting(AppSettings::ArgRequiredElseHelp))
        .subcommand(App::new("clr").about("Clear all saved favorites"))
        .subcommand(App::new("ls").about("Display all favorited branches and their keys"))
        .get_matches();

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
            config.state.insert(key, branch);
            config.save();
        }
        Some(("use", sub_matches)) => {
            let key = sub_matches
                .value_of("SHORTCUT_KEY")
                .expect("required")
                .trim()
                .to_string();
            match config.state.get(&key) {
                Some(branch_name) => switch_branch(branch_name).unwrap(),
                None => {
                    println!("No branch name set for key {}", key);
                    exit(1)
                }
            };
        }
        Some(("del", sub_matches)) => {
            let key = sub_matches
                .value_of("SHORTCUT_KEY")
                .expect("required")
                .trim()
                .to_string();
            config.state.remove(&key).unwrap();
            config.save()
        }
        Some(("ls", _)) => {
            config
                .state
                .into_iter()
                .for_each(|(k, v)| println!("{} => {}", k, v));
        }
        Some(("clr", _)) => {
            config.state = HashMap::new();
            config.save()
        }
        _ => unreachable!(),
    }
}

fn get_current_branch() -> String {
    let branch_name_raw = Command::new("sh")
        .arg("-c")
        .arg("git branch --show-current")
        .output()
        .expect("Failed to grab branch name");
    String::from_utf8(branch_name_raw.stdout).unwrap()
}

fn switch_branch(branch_name: &str) -> Result<ExitStatus, std::io::Error> {
    Command::new("sh")
        .arg("-c")
        .arg(format!("git checkout {:?}", branch_name))
        .status()
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Config {
    state: HashMap<String, String>,
    path_to_config: String,
}

impl Config {
    fn new(path: &str) -> Self {
        Config {
            state: HashMap::new(),
            path_to_config: path.to_string(),
        }
    }

    fn save(&self) {
        let ref file_path = Path::new(&self.path_to_config);
        let file = File::create(file_path).expect(
            format!(
                "Unable to create config file in {}",
                file_path.to_str().unwrap()
            )
            .as_str(),
        );
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &self).expect("Failed to save config")
    }

    fn load(&mut self) {
        let ref file_path = Path::new(&self.path_to_config);
        let file = match File::open(file_path) {
            Err(err) => {
                println!("Failed to open config: {:?}", err);
                println!("Creating new config {}", file_path.to_str().unwrap());
                let new_file = File::create(file_path).expect(
                    format!(
                        "Unable to create config file in {}",
                        file_path.to_str().unwrap()
                    )
                    .as_str(),
                );
                let writer = BufWriter::new(&new_file);
                serde_json::to_writer_pretty(writer, &self).expect("Failed to save config");
                File::open(file_path).expect("Unable to open newly created file")
            }
            Ok(file) => file,
        };

        let reader = BufReader::new(file);
        let config: Config = serde_json::from_reader(reader).expect("Failed to parse config");

        self.state = config.state;
        self.path_to_config = config.path_to_config
    }
}
