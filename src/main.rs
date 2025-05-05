mod config;
mod my_studio;

use std::{path::Path, process::exit};

use reqwest::Client;

#[tokio::main]
async fn main() {
    let config = config::load(Path::new("config.toml")).unwrap_or_else(|e| {
        eprintln!("Error: {e}");
        exit(1);
    });

    let client = Client::new();
}
