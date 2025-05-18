mod error;

use error::{Result, TomlError};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};

/// Configuration for the application.
///
/// This struct represents the application's configuration, which can be serialized to
/// and deserialized from a TOML file. It contains settings for themes, authentication
/// credentials, and other application preferences.
///
/// # Fields
///
/// * `theme` - The theme setting for the application's user interface.
/// * `my_studio` - Authentication credentials and settings for the MyStudio API.
/// * `config_path` - The path to the configuration file (not serialized to TOML).
///
/// # Example
///
/// ```rust
/// use std::path::Path;
/// use crate::config::{Config, load, Theme, MyStudio};
///
/// // Load an existing configuration or create a default one
/// let config_path = Path::new("config.toml");
/// let mut config = load(config_path).unwrap_or_default();
///
/// // Modify configuration values
/// config.theme = Theme::Dark;
/// config.my_studio = MyStudio {
///     email: "user@example.com".to_string(),
///     password: "secure_password".to_string(),
///     company_id: "12345".to_string(),
/// };
///
/// // Save the updated configuration
/// if let Err(e) = config.save() {
///     eprintln!("Failed to save configuration: {}", e);
/// }
/// ```
#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub theme: Theme,
    pub my_studio: MyStudio,

    #[serde(skip)]
    config_path: PathBuf,
}

/// Application user interface theme options.
///
/// This enum represents the available visual themes for the application interface.
/// The theme setting affects the color scheme and visual appearance throughout the application.
///
/// # Variants
///
/// * `System` - Uses the operating system's theme preference (default).
/// * `Dark` - Uses a dark color scheme with light text on dark backgrounds.
/// * `Light` - Uses a light color scheme with dark text on light backgrounds.
///
/// # Example
///
/// ```rust
/// use crate::config::Theme;
///
/// // Using the default theme (System)
/// let system_theme = Theme::default();
///
/// // Explicitly selecting a theme
/// let dark_theme = Theme::Dark;
/// let light_theme = Theme::Light;
///
/// // Using in configuration
/// let mut config = Config::default();
/// config.theme = Theme::Dark;
/// ```
#[derive(Serialize, Deserialize, Default)]
pub enum Theme {
    #[default]
    System,
    Dark,
    Light,
}

/// Authentication credentials and settings for the MyStudio API.
///
/// This struct contains the necessary information to authenticate with the MyStudio API,
/// including user credentials and company identification.
///
/// # Fields
///
/// * `email` - The user's email address used for authentication.
/// * `password` - The user's password used for authentication.
/// * `company_id` - The identifier for the user's company within the MyStudio system.
///
/// # Example
///
/// ```rust
/// use crate::config::MyStudio;
///
/// let credentials = MyStudio {
///     email: "user@example.com".to_string(),
///     password: "secure_password".to_string(),
///     company_id: "12345".to_string(),
/// };
///
/// // Use the credentials for API authentication
/// ```
#[derive(Serialize, Deserialize, Default)]
pub struct MyStudio {
    pub email: String,
    pub password: String,
    pub company_id: String,
}

impl Config {
    /// Saves the current configuration to its associated file.
    ///
    /// This method serializes the configuration to TOML format using pretty-printing,
    /// and writes it to the path stored in `config_path`. The method handles both
    /// the serialization to TOML and writing the file to disk.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the configuration was successfully saved, or an `Error`
    /// if serialization or file writing fails.
    ///
    /// # Errors
    ///
    /// This method can return the following errors:
    /// - `Error::Toml` if the configuration cannot be serialized to TOML.
    /// - `Error::Io` if the file cannot be written to disk.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::path::Path;
    /// use crate::config::load;
    ///
    /// let config_path = Path::new("config.toml");
    /// let mut config = load(config_path).unwrap_or_default();
    /// config.theme = Theme::Dark;
    ///
    /// if let Err(e) = config.save() {
    ///     eprintln!("Failed to save configuration: {}", e);
    /// } else {
    ///     println!("Configuration saved successfully");
    /// }
    /// ```
    pub fn save(&self) -> Result<()> {
        fs::write(
            &self.config_path,
            toml::to_string_pretty(self).map_err(TomlError::Serialize)?,
        )?;

        Ok(())
    }
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
/// };
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
