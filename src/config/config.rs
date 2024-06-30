use dirs::home_dir;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub api: String,
    pub token: String,
    pub model: String,
}

pub fn get_config_path() -> PathBuf {
    let mut config_path = home_dir().expect("Can't find dir!");
    config_path.push(".config/askshell/config.yaml");
    config_path
}

pub fn config_exists() -> bool {
    fs::metadata(get_config_path()).is_ok()
}

pub fn init_config() {
    let mut api = String::new();
    let mut token = String::new();
    let mut model = String::new();

    print!("Enter API URL: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut api).unwrap();
    print!("Enter API Token: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut token).unwrap();
    print!("Enter Model: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut model).unwrap();

    let config = Config {
        api: api.trim().to_string(),
        token: token.trim().to_string(),
        model: model.trim().to_string(),
    };

    let config_path = get_config_path();
    fs::create_dir_all(config_path.parent().unwrap()).unwrap();
    fs::write(config_path, serde_yaml::to_string(&config).unwrap()).unwrap();

    println!("Configuration saved.");
}

pub fn read_config() -> Option<Config> {
    let config_content = fs::read_to_string(get_config_path()).ok()?;
    let config: Config = serde_yaml::from_str(&config_content).ok()?;
    if !validate_config(&config) {
        eprintln!("Invalid configuration. Please reinitialize the configuration.");
        init_config();
        std::process::exit(1);
    }
    Some(config)
}

pub fn validate_config(config: &Config) -> bool {
    !config.api.trim().is_empty()
        && !config.token.trim().is_empty()
        && !config.model.trim().is_empty()
}
