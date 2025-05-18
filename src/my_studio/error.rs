use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

/// Represents errors that can occur in the `my_studio` module.
#[derive(Debug, Error)]
pub enum Error {
    /// An error returned by the API, such as missing fields or invalid requests.
    #[error(transparent)]
    Api(#[from] ApiError),

    /// An HTTP error that occurred during a request, originating from the `reqwest` library.
    #[error(transparent)]
    Http(#[from] reqwest::Error),

    /// A JSON parsing error, originating from the `serde_json` library.
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}

/// Represents API-specific errors that can occur in the `my_studio` module.
///
/// These errors are related to issues with the API response, such as missing fields,
/// invalid requests, or unrecognized values.
///
/// # Variants
///
/// * `InvalidRequest` - Represents an invalid request error returned by the API.
///   - `message` - A description of the error.
///   - `url` - The URL of the API endpoint that caused the error.
///
/// * `MissingField` - Indicates that a required field is missing or invalid in the API response.
///   - `field` - The name of the missing or invalid field.
///   - `url` - The URL of the API endpoint that returned the response.
///
/// * `UnrecognizedValue` - Represents an unrecognized or unexpected value in the API response.
///   - `field` - The name of the field containing the unrecognized value.
///   - `value` - The unrecognized value.
///   - `url` - The URL of the API endpoint that returned the response.
///
/// # Example
///
/// ```rust
/// use crate::my_studio::ApiError;
///
/// let error = ApiError::MissingField {
///     field: "status".to_string(),
///     url: "https://example.com/api".to_string(),
/// };
///
/// println!("Error: {error}");
/// ```
#[derive(Debug, Error)]
pub enum ApiError {
    /// Represents an invalid request error returned by the API.
    #[error("'{message}' received from call to {url}.")]
    InvalidRequest { message: String, url: String },

    /// Indicates that a required field is missing or invalid in the API response.
    #[error("Missing or invalid field '{field}' in response from call to {url}.")]
    MissingField { field: String, url: String },

    /// Represents an unrecognized or unexpected value in the API response.
    #[error("Unrecognized value '{value}' for field '{field}' in response from call to {url}.")]
    UnrecognizedValue {
        field: String,
        value: String,
        url: String,
    },
}
