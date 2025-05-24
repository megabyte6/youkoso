mod error;

use std::{
    fs,
    path::{Path, PathBuf},
};

use error::{Result, TomlError};
use serde::{Deserialize, Serialize};

use crate::spreadsheet::ColumnIndex;

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
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Config {
    pub theme: Theme,
    pub my_studio: MyStudio,
    pub student_data: StudentData,

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
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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
/// * `company_id` - The identifier for the user's company within the MyStudio system.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MyStudio {
    pub email: String,
    pub company_id: String,
}

/// Configuration for student data management.
///
/// This struct contains settings related to the source and structure of student data,
/// including file location, sheet identification, and column mappings for student information.
///
/// # Fields
///
/// * `filepath` - Path to the file containing student data.
/// * `sheet_name` - Name of the worksheet containing student records.
/// * `name_column` - Index of the column containing student names.
/// * `id_column` - Index of the column containing student identifiers.
/// * `immediate_sign_in` - Configuration for automatic sign-in functionality.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StudentData {
    pub filepath: PathBuf,
    pub sheet_name: String,
    pub name_column: ColumnIndex,
    pub id_column: ColumnIndex,
    pub immediate_sign_in: ImmediateSignIn,
}

/// Configuration for immediate sign-in functionality.
///
/// This struct defines settings that control automatic sign-in behavior,
/// specifying which column indicates eligibility and what symbol marks a student
/// as enabled for immediate sign-in.
///
/// # Fields
///
/// * `column` - Index of the column that indicates immediate sign-in eligibility.
/// * `enabled_symbol` - The string value that, when present in the column, enables immediate sign-in.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImmediateSignIn {
    pub column: ColumnIndex,
    pub enabled_symbol: String,
}

impl Default for ImmediateSignIn {
    fn default() -> Self {
        Self {
            column: Default::default(),
            enabled_symbol: "TRUE".to_owned(),
        }
    }
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
