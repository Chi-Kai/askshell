use crate::config::config::{config_exists, init_config, read_config, Config};
use crate::execute::execute::{execute_command, extract_command};
use crate::interface::request::send_request;
use clap::{Parser, Subcommand};

mod config;
mod execute;
mod interface;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<CommandEnum>,

    /// The content to send
    content: Option<String>,
}

#[derive(Subcommand, Debug)]
enum CommandEnum {
    /// Reinitialize configuration
    Config,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    if let Some(CommandEnum::Config) = args.command {
        init_config();
    } else {
        if !config_exists() {
            init_config();
        }
        let content = args.content.expect("Content is required");
        let config = read_config().expect("Failed to read config");
        let response = send_request(&config, &content).await;
        match response {
            Ok(response) => {
                let command = extract_command(&response);
                if let Some(command) = command {
                    execute_command(&command);
                } else {
                    eprintln!("No valid command found in the response.");
                }
            }
            Err(err) => eprintln!("Config Error: {}", err),
        }
    }
}
