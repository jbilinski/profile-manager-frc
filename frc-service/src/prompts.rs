
use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use std::error::Error;

#[derive(Deserialize, Clone)]
pub struct Prompts {
    pub resume_prompt: String,
    // Add more prompts as needed
}

impl Prompts {
    pub fn load_from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let prompts: Prompts = serde_json::from_str(&contents)?;
        Ok(prompts)
    }
}