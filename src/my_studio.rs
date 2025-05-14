use reqwest::Client;
use serde_json::{Value, json};
use thiserror::Error;

use crate::config::Config;

type Result<T> = std::result::Result<T, Error>;

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

/// Logs in to the MyStudio API.
///
/// This function sends a POST request to the MyStudio API to authenticate the user
/// and log in. It uses the provided `Client` and `Config` to construct the request
/// and handle the response.
///
/// # Arguments
///
/// * `client` - A `reqwest::Client` instance used to send the HTTP request.
/// * `config` - A `Config` struct containing the necessary credentials (email, password)
///   and company ID.
///
/// # Returns
///
/// Returns a `Result` containing `()` if the login is successful, or an `Error`
/// if an error occurs during the request or response handling.
///
/// # Errors
///
/// This function can return the following errors:
/// - `Error::Http` if an HTTP error occurs during the request.
/// - `Error::Json` if the response cannot be parsed as valid JSON.
/// - `Error::Api` if the API response contains an error, such as:
///   - Missing or invalid fields in the response.
///   - An unrecognized value in the response.
///
/// # Example
///
/// ```rust
/// use reqwest::Client;
/// use crate::config::Config;
/// use crate::my_studio::login;
///
/// #[tokio::main]
/// async fn main() {
///     let client = Client::new();
///     let config = Config {
///         my_studio: MyStudio {
///             email: "example@example.com".to_string(),
///             password: "password123".to_string(),
///             company_id: "12345".to_string(),
///         },
///     };
///
///     match login(client, config).await {
///         Ok(_) => println!("Login successful!"),
///         Err(e) => eprintln!("Error: {e}"),
///     }
/// }
/// ```
pub async fn login(client: Client, config: Config) -> Result<()> {
    let request_url = "https://cn.mystudio.io/Api/v2/login";
    let request_body = &json!({
        "email": config.my_studio.email,
        "password": config.my_studio.password,
        "from_page": "attendance"
    });

    let response: Value = client
        .post(request_url)
        .json(request_body)
        .send()
        .await?
        .json()
        .await?;

    let status = response["status"].as_str().ok_or(ApiError::MissingField {
        field: "status".to_owned(),
        url: request_url.to_owned(),
    })?;

    match status {
        "Success" => Ok(()),
        "Failed" => {
            let msg = response["status"]["msg"]
                .as_str()
                .ok_or(ApiError::MissingField {
                    field: "msg".to_owned(),
                    url: request_url.to_owned(),
                })?;
            Err(Error::Api(ApiError::InvalidRequest {
                message: msg.to_owned(),
                url: request_url.to_owned(),
            }))
        }
        _ => Err(Error::Api(ApiError::UnrecognizedValue {
            field: "status".to_owned(),
            value: status.to_owned(),
            url: request_url.to_owned(),
        })),
    }
}

/// Retrieves a session token from the MyStudio API.
///
/// This function sends a POST request to the MyStudio API to generate a session token
/// for attendance purposes. It uses the provided `Client` and `Config` to construct
/// the request and handle the response.
///
/// # Arguments
///
/// * `client` - A `reqwest::Client` instance used to send the HTTP request.
/// * `config` - A `Config` struct containing the necessary credentials and company ID.
///
/// # Returns
///
/// Returns a `Result` containing the session token as a `String` if successful, or an `Error`
/// if an error occurs during the request or response handling.
///
/// # Errors
///
/// This function can return the following errors:
/// - `Error::Http` if an HTTP error occurs during the request.
/// - `Error::Json` if the response cannot be parsed as valid JSON.
/// - `Error::Api` if the API response contains an error, such as:
///   - Missing or invalid fields in the response.
///   - An unrecognized value in the response.
///
/// # Example
///
/// ```rust
/// use reqwest::Client;
/// use crate::config::Config;
/// use crate::my_studio::get_session_token;
///
/// #[tokio::main]
/// async fn main() {
///     let client = Client::new();
///     let config = Config {
///         my_studio: MyStudio {
///             email: "example@example.com".to_string(),
///             password: "password123".to_string(),
///             company_id: "12345".to_string(),
///         },
///     };
///
///     match get_session_token(client, config).await {
///         Ok(token) => println!("Session token: {token}"),
///         Err(e) => eprintln!("Error: {e}"),
///     }
/// }
/// ```
pub async fn get_session_token(client: Client, config: Config) -> Result<String> {
    let request_url = "https://cn.mystudio.io/Api/v2/generateStudioAttendanceToken";
    let request_body = &json!({
        "company_id": config.my_studio.company_id,
        "email": config.my_studio.email,
        "from_page": "attendance"
    });

    let response: Value = client
        .post(request_url)
        .json(request_body)
        .send()
        .await?
        .json()
        .await?;

    let status = response["status"].as_str().ok_or(ApiError::MissingField {
        field: "status".to_owned(),
        url: request_url.to_owned(),
    })?;

    match status {
        "Success" => {
            let msg = response["msg"].as_str().ok_or(ApiError::MissingField {
                field: "msg".to_owned(),
                url: request_url.to_owned(),
            })?;
            Ok(msg.to_string())
        }
        "Failed" => {
            let msg = response["msg"].as_str().ok_or(ApiError::MissingField {
                field: "msg".to_owned(),
                url: request_url.to_owned(),
            })?;
            Err(Error::Api(ApiError::InvalidRequest {
                message: msg.to_owned(),
                url: request_url.to_owned(),
            }))
        }
        _ => Err(Error::Api(ApiError::UnrecognizedValue {
            field: "status".to_owned(),
            value: status.to_owned(),
            url: request_url.to_owned(),
        })),
    }
}
