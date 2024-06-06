use crate::Config;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde_json::json;

pub async fn send_request(config: &Config, content: &str) -> Result<String, reqwest::Error> {
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
