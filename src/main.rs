mod config;

use std::{path::Path, process::exit};

fn main() {
    let config = config::load(Path::new("config.toml")).unwrap_or_else(|e| {
        eprintln!("Error: {e}");
        exit(1);
    });
}
