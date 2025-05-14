use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};
use thiserror::Error;

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub theme: Theme,
    pub my_studio: MyStudio,

    #[serde(skip)]
    config_path: PathBuf,
}

#[derive(Serialize, Deserialize, Default)]
pub enum Theme {
    #[default]
    System,
    Dark,
    Light,
}

#[derive(Serialize, Deserialize, Default)]
pub struct MyStudio {
    pub email: String,
    pub password: String,
    pub company_id: String,
}

impl Config {
    pub fn save(&self) -> Result<()> {
        fs::write(
            &self.config_path,
            toml::to_string_pretty(self).map_err(TomlError::Serialize)?,
        )?;

        Ok(())
    }
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Toml(#[from] TomlError),
}

#[derive(Debug, Error)]
pub enum TomlError {
    #[error(transparent)]
    Serialize(#[from] toml::ser::Error),

    #[error(transparent)]
    Deserialize(#[from] toml::de::Error),
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
pub fn load(config_path: &Path) -> Result<Config> {
    let mut config: Config;

    if config_path.exists() {
        let contents = fs::read_to_string(config_path)?;
        config = toml::from_str(&contents).map_err(TomlError::Deserialize)?;
    } else {
        config = Config::default();
        let default_config = toml::to_string(&config).map_err(TomlError::Serialize)?;
        fs::write(config_path, default_config)?;
    }

    config.config_path = config_path.to_path_buf();

    Ok(config)
}
