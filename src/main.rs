use crate::config::config::{config_exists, init_config, read_config, Config};
use crate::execute::execute::{execute_command, extract_command};
use crate::interface::request::send_request;
use clap::Parser;

mod config;
mod execute;
mod interface;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The content to send
    content: String,
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
