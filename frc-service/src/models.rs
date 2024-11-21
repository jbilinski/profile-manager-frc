// Data Sctructs and SerDe logic

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct Resume {
    pub name: String,
    pub email: String,
    pub experience: Vec<String>,
}

pub fn generate_struct_from_json(schema: &Value) {
    if let Some(properties) = schema.get("properties") {
        println!("#[derive(Serialize, Deserialize, Debug)]");
        println!("struct {} {{", schema["title"].as_str().unwrap_or("GeneratedStruct"));

        for (key, value) in properties.as_object().unwrap() {
            let rust_type = match value["type"].as_str().unwrap() {
                "string" => "String",
                "integer" => "i32",
                "boolean" => "bool",
                "array" => "Vec<String>", // Simplified for arrays of strings
                "object" => "serde_json::Value", // Simplified for nested objects
                _ => "String", // Default to String for unknown types
            };

            let is_optional = !schema["required"].as_array().unwrap_or(&vec![]).contains(&Value::String(key.clone()));
            if is_optional {
                println!("    {}: Option<{}>,", key, rust_type);
            } else {
                println!("    {}: {},", key, rust_type);
            }
        }

        println!("}}");
    }
}