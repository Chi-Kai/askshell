use clap::Parser;
use dirs::home_dir;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The content to send
    content: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    api: String,
    token: String,
}

fn get_config_path() -> PathBuf {
    let mut config_path = home_dir().expect("Can't find dir!");
    config_path.push(".config/askshell/config.yaml");
    config_path
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    if !config_exists() {
        init_config();
    }

    let config = read_config().expect("Failed to read config");
    let response = send_request(&config, &args.content).await;
    match response {
        Ok(response) => {
            let command = extract_command(&response);
            if let Some(command) = command {
                execute_command(&command);
            } else {
                eprintln!("No valid command found in the response.");
            }
        }
        Err(err) => eprintln!("Error: {}", err),
    }
}

fn config_exists() -> bool {
    fs::metadata(get_config_path()).is_ok()
}

fn init_config() {
    let mut api = String::new();
    let mut token = String::new();

    print!("Enter API URL: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut api).unwrap();
    print!("Enter API Token: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut token).unwrap();

    let config = Config {
        api: api.trim().to_string(),
        token: token.trim().to_string(),
    };

    let config_path = get_config_path();
    fs::create_dir_all(config_path.parent().unwrap()).unwrap();
    fs::write(config_path, serde_yaml::to_string(&config).unwrap()).unwrap();

    println!("Configuration saved.");
}

fn read_config() -> Option<Config> {
    let config_content = fs::read_to_string(get_config_path()).ok()?;
    let config: Config = serde_yaml::from_str(&config_content).ok()?;
    Some(config)
}

async fn send_request(config: &Config, content: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let url = format!("https://{}/v1/chat/completions", config.api);

    let prompt = format!(
        "我希望你充当智能linux终端。我将键入我的需求，您根据我的要求输出相应的指令。我希望您只在一个唯一的代码块内回复终端输出，而不是其他任何内容。不要写解释。除非我指示您这样做。我的第一个命令是 {}",
        content
    );

    let res = client
        .post(&url)
        .header(CONTENT_TYPE, "application/json")
        .header(AUTHORIZATION, format!("Bearer {}", config.token))
        .json(&json!({
            "model": "yi-large",
            "messages": [{"role": "user", "content": prompt}],
            "temperature": 0.3
        }))
        .send()
        .await?;

    let text = res.text().await?;
    Ok(text)
}

fn extract_command(response: &str) -> Option<String> {
    let json: serde_json::Value = serde_json::from_str(response).ok()?;
    let content = json["choices"][0]["message"]["content"].as_str()?;

    // Extract command between code block (```)
    let command_start = content.find("```")?;
    let command_end = content[command_start + 3..].find("```")? + command_start + 3;
    let command = content[command_start + 3..command_end].trim().to_string();

    // Remove prefix bash\n$
    let command = command.trim_start_matches("bash\n$").trim();

    Some(command.to_string())
}

fn execute_command(command: &str) {
    println!("Executing command: {}", command);
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect("Failed to execute command");

    // debug
    // println!("{}", String::from_utf8_lossy(&output.stdout));
    if !output.stderr.is_empty() {
        println!("Get command: {}", command);
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
    }
}
