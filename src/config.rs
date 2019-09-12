use serde::Deserialize;
use std::fs;
use std::io::ErrorKind;
use std::path::PathBuf;

use failure::Error;
use toml;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default = "default_osci")]
    pub osci_url: String,
    pub user: Option<String>,
    pub password: Option<String>,
}

impl Config {
    pub fn load(input: &PathBuf) -> Result<Config, Error> {
        match fs::read_to_string(input) {
            Ok(input) => Ok(toml::from_str(&input)?),
            Err(e) => match e.kind() {
                ErrorKind::NotFound => Ok(toml::from_str("")?),
                _ => Err(e.into()),
            },
        }
    }
}

fn default_osci() -> String {
    "http://10.245.162.58:8080/".to_string()
}
