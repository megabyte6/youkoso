use std::{cell::RefCell, rc::Rc};

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

/// An HTTP client for interacting with the MyStudio API.
///
/// This struct encapsulates the functionality needed to communicate with the MyStudio API,
/// including authentication, session management, and request handling. It maintains a
/// reusable HTTP client, configuration data, and the current session token state.
///
/// # Fields
///
/// * `client` - A `reqwest::Client` instance used for making HTTP requests to the API.
/// * `config` - A shared, mutable reference to a `Config` struct containing authentication
///   credentials and other settings.
/// * `session_token` - An optional String that stores the session token after successful
///   authentication.
///
/// # Examples
///
/// ```rust
/// use std::cell::RefCell;
/// use std::rc::Rc;
/// use crate::config::{Config, MyStudio};
/// use crate::my_studio::HttpClient;
///
/// // Create a configuration
/// let config = Config {
///     my_studio: MyStudio {
///         email: "example@example.com".to_string(),
///         password: "password123".to_string(),
///         company_id: "12345".to_string(),
///     },
///     // other config fields...
/// };
///
/// // Create a shared reference to the config
/// let config_rc = Rc::new(RefCell::new(config));
///
/// // Create the HTTP client
/// let client = HttpClient::new(config_rc);
///
/// // Use the client to make API calls
/// ```
pub struct HttpClient {
    client: Client,
    config: Rc<RefCell<Config>>,
    session_token: Option<String>,
}

impl HttpClient {
    /// Creates a new HTTP client for interacting with the MyStudio API.
    ///
    /// This constructor initializes an `HttpClient` instance with the provided configuration.
    /// It creates a new `reqwest::Client` for making HTTP requests and initializes the
    /// session token as `None`. The session token will be populated after a successful login
    /// or token acquisition.
    ///
    /// # Arguments
    ///
    /// * `config` - A shared, mutable reference to a `Config` struct wrapped in `Rc<RefCell<>>`,
    ///   containing the necessary credentials and settings for API authentication.
    ///
    /// # Returns
    ///
    /// Returns a new `HttpClient` instance configured with the provided settings.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::cell::RefCell;
    /// use std::rc::Rc;
    /// use crate::config::{Config, MyStudio};
    /// use crate::my_studio::HttpClient;
    ///
    /// let config = Config {
    ///     my_studio: MyStudio {
    ///         email: "example@example.com".to_string(),
    ///         password: "password123".to_string(),
    ///         company_id: "12345".to_string(),
    ///     },
    /// };
    ///
    /// let config_rc = Rc::new(RefCell::new(config));
    /// let client = HttpClient::new(config_rc);
    /// ```
    pub fn new(config: Rc<RefCell<Config>>) -> HttpClient {
        Self {
            client: Client::new(),
            config,
            session_token: None,
        }
    }

    /// Logs in to the MyStudio API.
    ///
    /// This method sends a POST request to the MyStudio API to authenticate the user
    /// and log in. It uses the client instance and configuration stored in the `HttpClient`.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing `()` if the login is successful, or an `Error`
    /// if an error occurs during the request or response handling.
    ///
    /// # Errors
    ///
    /// This method can return the following errors:
    /// - `Error::Http` if an HTTP error occurs during the request.
    /// - `Error::Json` if the response cannot be parsed as valid JSON.
    /// - `Error::Api` if the API response contains an error, such as:
    ///   - Missing or invalid fields in the response.
    ///   - An unrecognized value in the response.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::cell::RefCell;
    /// use std::rc::Rc;
    /// use crate::config::{Config, MyStudio};
    /// use crate::my_studio::HttpClient;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = Config {
    ///         my_studio: MyStudio {
    ///             email: "example@example.com".to_string(),
    ///             password: "password123".to_string(),
    ///             company_id: "12345".to_string(),
    ///         },
    ///     };
    ///     
    ///     let config_rc = Rc::new(RefCell::new(config));
    ///     let client = HttpClient::new(config_rc);
    ///
    ///     match client.login().await {
    ///         Ok(_) => println!("Login successful!"),
    ///         Err(e) => eprintln!("Error: {e}"),
    ///     };
    /// }
    /// ```
    pub async fn login(&self) -> Result<()> {
        let request_url = "https://cn.mystudio.io/Api/v2/login";
        let request_body = &json!({
            "email": self.config.try_borrow().unwrap().my_studio.email,
            "password": self.config.try_borrow().unwrap().my_studio.password,
            "from_page": "attendance"
        });

        let response: Value = self
            .client
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
    /// This method sends a POST request to the MyStudio API to generate a session token
    /// for attendance purposes. It uses the client instance stored in the `HttpClient`
    /// and the provided `Config` parameter.
    ///
    /// # Arguments
    ///
    /// * `config` - A `Config` struct containing the necessary credentials and company ID.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the session token as a `String` if successful, or an `Error`
    /// if an error occurs during the request or response handling.
    ///
    /// # Errors
    ///
    /// This method can return the following errors:
    /// - `Error::Http` if an HTTP error occurs during the request.
    /// - `Error::Json` if the response cannot be parsed as valid JSON.
    /// - `Error::Api` if the API response contains an error, such as:
    ///   - Missing or invalid fields in the response.
    ///   - An unrecognized value in the response.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::cell::RefCell;
    /// use std::rc::Rc;
    /// use crate::config::{Config, MyStudio};
    /// use crate::my_studio::HttpClient;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = Config {
    ///         my_studio: MyStudio {
    ///             email: "example@example.com".to_string(),
    ///             password: "password123".to_string(),
    ///             company_id: "12345".to_string(),
    ///         },
    ///     };
    ///     
    ///     let config_rc = Rc::new(RefCell::new(config.clone()));
    ///     let client = HttpClient::new(config_rc);
    ///
    ///     match client.aquire_session_token(config).await {
    ///         Ok(token) => println!("Session token: {token}"),
    ///         Err(e) => eprintln!("Error: {e}"),
    ///     };
    /// }
    /// ```
    pub async fn aquire_session_token(&self, config: Config) -> Result<String> {
        let request_url = "https://cn.mystudio.io/Api/v2/generateStudioAttendanceToken";
        let request_body = &json!({
            "company_id": config.my_studio.company_id,
            "email": config.my_studio.email,
            "from_page": "attendance"
        });

        let response: Value = self
            .client
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
}
