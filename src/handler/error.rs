//! Error components for the library.

use std::result;
use thiserror::Error;

/// A type alias for handling errors related to rustnao.
pub type Result<T> = result::Result<T, Error>;

/// An error that can occur while interacting to the SauceNAO API.
#[derive(Debug, Error)]
pub enum Error {
    /// An error when forming the URL for the API.
    #[error("ERROR: URL was invalid, error was due to: {0}")]
    InvalidParse(String),
    /// An error when getting the file path of a file for the API.
    #[error("ERROR: File path was invalid, error was due to: {0}")]
    InvalidFile(String),
    /// An error when trying to deserialize the resulting JSON from the API.
    #[error("ERROR: Could not properly serde results: {0}")]
    InvalidSerde(String),
    /// An error when receiving an unsuccessful code from the SauceNAO API.
    #[error("ERROR: Recieved an invalid status code {code} after API call with message: \"{message}\"")]
    InvalidCode {
        /// The error code from SauceNAO.
        code: i32,
        /// The message showing the cause of the error from SauceNAO.
        message: String,
    },
    /// An error when trying to send an invalid request to the API.
    #[error("ERROR: Failed to make the request, error was due to: {0}")]
    InvalidRequest(String),
    /// An error with some data that is passed in by the user.
    #[error("ERROR: An invalid parameter was passed, error was due to: {0}")]
    InvalidParameters(String),
}

impl Error {
	pub(crate) fn invalid_parse<T: AsRef<str>>(unk: T) -> Error {
		Error::InvalidParse(unk.as_ref().to_string())
	}

	pub(crate) fn invalid_path<T: AsRef<str>>(unk: T) -> Error {
		Error::InvalidFile(unk.as_ref().to_string())
	}

	pub(crate) fn invalid_serde<T: AsRef<str>>(unk: T) -> Error {
		Error::InvalidSerde(unk.as_ref().to_string())
	}

	pub(crate) fn invalid_code(code: i32, message: String) -> Error {
		Error::InvalidCode { code, message }
	}

	pub(crate) fn invalid_request<T: AsRef<str>>(unk: T) -> Error {
		Error::InvalidRequest(unk.as_ref().to_string())
	}

	pub(crate) fn invalid_parameter(message: String) -> Error {
		Error::InvalidParameters(message)
	}
}

impl From<serde_json::Error> for Error {
	fn from(err: serde_json::Error) -> Self {
		Error::invalid_serde(err.to_string())
	}
}

impl From<url::ParseError> for Error {
	fn from(err: url::ParseError) -> Self {
		Error::invalid_parse(err.to_string())
	}
}

impl From<std::num::ParseIntError> for Error {
	fn from(err: std::num::ParseIntError) -> Self {
		Error::invalid_parse(err.to_string())
	}
}

impl From<std::num::ParseFloatError> for Error {
	fn from(err: std::num::ParseFloatError) -> Self {
		Error::invalid_parse(err.to_string())
	}
}

impl From<std::io::Error> for Error {
	fn from(err: std::io::Error) -> Self {
		Error::invalid_path(err.to_string())
	}
}

impl From<reqwest::Error> for Error {
	fn from(err: reqwest::Error) -> Self {
		Error::invalid_request(err.to_string())
	}
}
