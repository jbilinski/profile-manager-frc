//incoming HTTP request parsing and response generation

use actix_web::{web, Responder};
use crate::models::Resume;
use crate::services::apicall_openai::call_gpt4o_api;
use crate::prompts::Prompts;
use std::env;
use reqwest;
use serde_json::Value;
use std::error::Error;

pub async fn generate_resume(resume: web::Json<Resume>, prompts: web::Data<Prompts>) -> impl Responder {
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
    let prompt = prompts.resume_prompt
        .replace("{name}", &resume.name)
        .replace("{experience}", &resume.experience.join(", "));

    match call_gpt4o_api(&prompt, &api_key).await {
        Ok(response) => web::Json(response),
        Err(_) => web::Json(serde_json::json!({"error": "Failed to call GPT-4o API"})),
    }
}

pub async fn fetch_json_schema(url: &str) -> Result<Value, Box<dyn Error>> {
    let response = reqwest::get(url).await?;
    let schema: Value = response.json().await?;
    Ok(schema)
}