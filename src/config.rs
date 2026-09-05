use etcetera::{BaseStrategy, choose_base_strategy};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub host: String,
    pub model: String,
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

pub fn save_config(host: &str, model: &str) {
    let config = Config {
        host: host.to_string(),
        model: model.to_string(),
    };

    write_config(&config);
}

fn write_config(config: &Config) {
    let path = get_config_path();

    if let Ok(content) = toml::to_string_pretty(config) {
        fs::write(&path, content).unwrap();
        println!("Saved config to: {:?}", path);
    }
}

pub fn save_host(new_host: &str) {
    let mut config = load_config().unwrap_or(Config {
        host: String::new(),
        model: String::new(),
    });

    config.host = new_host.to_string();
    write_config(&config);
}

pub fn save_model(new_model: &str) {
    let mut config = load_config().unwrap_or(Config {
        host: String::new(),
        model: String::new(),
    });

    config.model = new_model.to_string();
    write_config(&config);
}
