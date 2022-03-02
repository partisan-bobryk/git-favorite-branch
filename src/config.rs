use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, BufWriter},
    path::Path,
};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Config {
    pub state: HashMap<String, String>,
    pub path_to_config: String,
}

impl Config {
    pub fn new(path: &str, state: HashMap<String, String>) -> Self {
        Config {
            state,
            path_to_config: path.to_string(),
        }
    }

    pub fn save(&self) {
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

    pub fn load(&mut self) {
        let file_path = Path::new(&self.path_to_config);
        let file = self.create_file_if_none_exist(file_path);

        let reader = BufReader::new(file);
        let config: Config = serde_json::from_reader(reader).expect("Failed to parse config");

        self.state = config.state;
        self.path_to_config = config.path_to_config
    }

    fn create_file_if_none_exist(&self, formatted_file_path: &Path) -> File {
        let file = match File::open(formatted_file_path) {
            Err(err) => {
                println!("Failed to open config: {:?}", err);
                println!(
                    "Creating new config {}",
                    formatted_file_path.to_str().unwrap()
                );
                let new_file = File::create(formatted_file_path).expect(
                    format!(
                        "Unable to create config file in {}",
                        formatted_file_path.to_str().unwrap()
                    )
                    .as_str(),
                );
                let writer = BufWriter::new(&new_file);
                serde_json::to_writer_pretty(writer, &self).expect("Failed to save config");
                File::open(formatted_file_path).expect("Unable to open newly created file")
            }
            Ok(file) => file,
        };

        file
    }
}
