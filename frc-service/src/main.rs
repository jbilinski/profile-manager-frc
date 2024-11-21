use actix_web::{App, HttpServer};
use dotenv::dotenv;
use std::env;
use log::info;
use crate::prompts::Prompts;

mod handlers;
mod models;
mod routes;
mod prompts;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let bind_ip = env::var("BIND_IP").unwrap_or_else(|_| "127.0.0.1".to_string());
    let bind_port = env::var("BIND_PORT").unwrap_or_else(|_| "9086".to_string());
    let bind_address = format!("{}:{}", bind_ip, bind_port);

    info!("Starting server at {}", bind_address);

    let jsonresume_schema_url = "https://github.com/jsonresume/resume-schema/raw/refs/heads/master/schema.json";
    let jsonresume_schema = fetch_and_generate_schema(jsonresume_schema_url).await?;

    let prompts = Prompts::load_from_file("frc-service/prompts.json")
        .expect("Failed to load prompts");

    start_server(&bind_address, prompts).await
}

async fn fetch_and_generate_schema(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let jsonresume_schema = handlers::fetch_json_schema(url).await?;
    models::generate_struct_from_json(&jsonresume_schema);
    Ok(jsonresume_schema)
}

async fn start_server(bind_address: &str, prompts: Prompts) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(prompts.clone()))
            .configure(routes::init)
    })
    .bind(bind_address)?
    .run()
    .await
}



