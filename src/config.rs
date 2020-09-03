use serde::{Deserialize, Serialize};
use serde_yaml;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub token: String,
    pub prefix: String,
    pub db_uri: String,
    location: String
}

impl Config {
    pub fn new(location: String) -> Config {
        match Config::retrieve(&location) {
            Some(conf) => conf,
            None => {
                let conf = Config {
                    token: String::new(),
                    prefix: String::from(";"),
                    db_uri: String::new(),
                    location,
                };
                conf.save();
                panic!(
                    "Saved a new config, please fill it out \n{}",
                    &conf.location,
                );
            }
        }
    }

    pub fn save(&self) {
        let serialized = serde_yaml::to_string(&self).expect("Failed to serialize");
        match File::create(&self.location) {
            Ok(mut file) => {
                file.write_all(serialized.as_bytes())
                    .expect("Failed to write")
            }
            Err(e) => {
                panic!("Failed to save config at {}\n{}", self.location, e)
            }
        }
    }

    fn retrieve(location: &String) -> Option<Config> {
        match File::open(location) {
            Ok(mut file) => {
                let mut contents = String::new();
                if let Err(_) = file.read_to_string(&mut contents) {
                    return None;
                };

                match serde_yaml::from_str(&contents) {
                    Ok(des) => Some(des),
                    Err(_) => None
                }
            }
            Err(_) => None
        }
    }
}