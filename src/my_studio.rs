mod error;

use std::{cell::RefCell, rc::Rc};

use error::{ApiError, Error, Result};
use reqwest::Client;
use serde_json::{Value, json};

use crate::config::Config;

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
#[derive(Debug, Clone)]
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
    pub fn new(config: Rc<RefCell<Config>>) -> HttpClient {
        Self {
            client: Client::new(),
            config,
            session_token: None,
        }
    }

    /// Retrieves a session token from the MyStudio API.
    ///
    /// This method sends a POST request to the MyStudio API to generate a session token
    /// for attendance purposes. It uses the client instance stored in the `HttpClient`.
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
    pub async fn aquire_session_token(&self) -> Result<String> {
        let request_url = "https://cn.mystudio.io/Api/v2/generateStudioAttendanceToken";
        let request_body = &json!({
            "company_id": self.config.try_borrow()?.my_studio.company_id,
            "email": self.config.try_borrow()?.my_studio.email,
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
