use reqwest::Client;
use serde_json::Value;
use std::error::Error;

pub async fn call_gpt4o_api(prompt: &str, api_key: &str) -> Result<Value, Box<dyn Error>> {
    let client = Client::new();
    let response = client.post("https://api.openai.com/v1/engines/gpt-4o/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&serde_json::json!({
            "prompt": prompt,
            "max_tokens": 150
        }))
        .send()
        .await?
        .json::<Value>()
        .await?;
    
    Ok(response)
}