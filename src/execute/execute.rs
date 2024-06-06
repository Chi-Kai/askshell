use std::process::Command;

pub fn extract_command(response: &str) -> Option<String> {
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

pub fn execute_command(command: &str) {
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
