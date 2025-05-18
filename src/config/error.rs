use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

/// Represents errors that can occur in the configuration module.
///
/// This enum encompasses all types of errors that might arise during
/// configuration operations, such as file I/O operations or TOML
/// serialization/deserialization.
///
/// # Variants
///
/// * `Io` - Represents I/O errors that occur during file operations.
/// * `Toml` - Represents errors related to TOML serialization or deserialization.
///
/// # Example
///
/// ```rust
/// use std::path::Path;
/// use crate::config::{self, Error};
///
/// let result = config::load(Path::new("non_existent_path/config.toml"));
/// if let Err(Error::Io(io_error)) = result {
///     eprintln!("I/O error occurred: {}", io_error);
/// }
/// ```
#[derive(Debug, Error)]
pub enum Error {
    /// Represents I/O errors that occur during file operations.
    ///
    /// This variant wraps `std::io::Error` and is used when operations like
    /// reading from or writing to configuration files fail.
    #[error(transparent)]
    Io(#[from] std::io::Error),

    /// Represents errors related to TOML serialization or deserialization.
    ///
    /// This variant wraps `TomlError` and is used when the configuration
    /// cannot be serialized to or deserialized from TOML format.
    #[error(transparent)]
    Toml(#[from] TomlError),
}

/// Represents specific errors related to TOML operations.
///
/// This enum distinguishes between serialization and deserialization errors
/// that can occur when working with TOML data.
///
/// # Variants
///
/// * `Serialize` - Represents errors that occur when serializing data to TOML format.
/// * `Deserialize` - Represents errors that occur when deserializing data from TOML format.
///
/// # Example
///
/// ```rust
/// use crate::config::{Config, TomlError};
///
/// let invalid_toml = "theme = [invalid]";
/// let result = toml::from_str::<Config>(invalid_toml);
/// if let Err(toml_err) = result {
///     let error = TomlError::Deserialize(toml_err);
///     eprintln!("Failed to parse TOML: {}", error);
/// }
/// ```
#[derive(Debug, Error)]
pub enum TomlError {
    /// Represents errors that occur when serializing data to TOML format.
    ///
    /// This variant wraps `toml::ser::Error` and is used when a Rust data structure
    /// cannot be converted to TOML format.
    #[error(transparent)]
    Serialize(#[from] toml::ser::Error),

    /// Represents errors that occur when deserializing data from TOML format.
    ///
    /// This variant wraps `toml::de::Error` and is used when TOML data
    /// cannot be parsed into the expected Rust data structure.
    #[error(transparent)]
    Deserialize(#[from] toml::de::Error),
}
