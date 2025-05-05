use serde::{Deserialize, Serialize};
use std::{error::Error, fs, path::Path};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub my_studio: MyStudio,
}

#[derive(Serialize, Deserialize)]
pub struct MyStudio {
    pub email: String,
    pub password: String,
    pub company_id: String,
}

/// Loads the configuration from a TOML file at the specified path.
///
/// If the file exists, it reads the contents and attempts to parse it as a TOML configuration.
/// If the file does not exist, it creates a default configuration, serializes it to TOML,
/// and writes it to the specified path.
///
/// # Arguments
///
/// * `config_path` - A reference to a `Path` that specifies the location of the configuration file.
///
/// # Returns
///
/// Returns a `Result` containing the `Config` struct if successful, or a boxed `dyn Error` if an error occurs.
///
/// # Errors
///
/// This function can return an error in the following cases:
/// - If the file exists but cannot be read.
/// - If the file contents cannot be parsed as valid TOML.
/// - If the default configuration cannot be serialized to TOML.
/// - If the default configuration cannot be written to the specified path.
///
/// # Example
///
/// ```rust
/// use std::path::Path;
/// use crate::config::load;
///
/// let config_path = Path::new("config.toml");
/// match load(config_path) {
///     Ok(config) => println!("Configuration loaded successfully!"),
///     Err(e) => eprintln!("Failed to load configuration: {e}"),
/// }
/// ```
pub fn load(config_path: &Path) -> Result<Config, Box<dyn Error>> {
    let config: Config;

    if config_path.exists() {
        let contents = fs::read_to_string(config_path)
            .map_err(|e| format!("Could not read file `{}`\n{e}", config_path.display()))?;

        config = toml::from_str(&contents).map_err(|e| {
            format!(
                "Unable to parse TOML config from `{}`\n{e}",
                config_path.display()
            )
        })?;
    } else {
        config = Config {
            my_studio: MyStudio {
                email: String::new(),
                password: String::new(),
                company_id: String::new(),
            },
        };

        let default_config_toml = toml::to_string(&config)
            .map_err(|e| format!("Failed to serialize default config into TOML\n{e}"))?;

        fs::write(config_path, default_config_toml).map_err(|e| {
            format!(
                "Failed to write default config to `{}`\n{e}",
                config_path.display()
            )
        })?;
    }

    Ok(config)
}
