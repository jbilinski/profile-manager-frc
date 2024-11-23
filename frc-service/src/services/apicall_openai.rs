use reqwest::Client;
use serde_json::Value;
use std::error::Error;

pub async fn call_gpt4o_api(prompt: &str, api_key: &str) -> Result<Value, Box<dyn Error>> {
    let client = Client::new();
    let response = client.post("https://api.openai.com/v1/engines/davinci-codex/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&serde_json::json!({
            "prompt": prompt,
            "max_tokens": 50
        }))
        .send()
        .await?;

    if response.status().is_success() {
        let response_json: Value = response.json().await?;
        Ok(response_json)
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to call GPT-4o API",
        )))
    }
}