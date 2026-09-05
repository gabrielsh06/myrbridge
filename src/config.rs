use etcetera::{BaseStrategy, choose_base_strategy};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub host: String,
}

fn get_config_path() -> PathBuf {
    let strategy = choose_base_strategy().unwrap();
    let directory = strategy.config_dir().join("myrbridge");
    fs::create_dir_all(&directory).unwrap();

    directory.join("config.toml")
}

pub fn load_config() -> Option<Config> {
    let path = get_config_path();

    if path.exists() {
        let content = fs::read_to_string(&path).ok()?;
        toml::from_str(&content).ok()
    } else {
        None
    }
}

pub fn save_config(host: &str) {
    let path = get_config_path();

    let config = Config {
        host: host.to_string(),
    };

    if let Ok(content) = toml::to_string_pretty(&config) {
        fs::write(&path, content).unwrap();
        println!("Saved config to: {:?}", path);
    }
}
