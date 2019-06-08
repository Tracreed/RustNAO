//! Error components for the library.
//! Much thanks to Andrew Gallant for the basis of this part of the library... followed the following
//! code for this: https://github.com/BurntSushi/imdb-rename/blob/master/imdb-index/src/error.rs

use std::fmt;
use std::result;
use failure::{Backtrace, Context, Fail};

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
	context : Context<ErrType>,
}

impl Error {
	pub fn kind(&self) -> &ErrType {
		self.context.get_context()
	}

	pub(crate) fn invalid_url<T: AsRef<str>>(unk: T) -> Error {
		Error::from(ErrType::InvalidURL(unk.as_ref().to_string()))
	}

	pub(crate) fn invalid_serde<T: AsRef<str>>(unk: T) -> Error {
		Error::from(ErrType::InvalidSerde(unk.as_ref().to_string()))
	}

	pub(crate) fn invalid_code(code : i32, message : String) -> Error {
		Error::from(ErrType::InvalidCode{code, message})
	}

	pub(crate) fn invalid_request<T: AsRef<str>>(unk: T) -> Error {
		Error::from(ErrType::InvalidRequest(unk.as_ref().to_string()))
	}
}

impl Fail for Error {
	fn cause(&self) -> Option<&Fail> {
		self.context.cause()
	}

	fn backtrace(&self) -> Option<&Backtrace> {
		self.context.backtrace()
	}
}

impl fmt::Display for Error {
	fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
		self.context.fmt(f)
	}
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ErrType {
	InvalidURL(String),
	InvalidSerde(String),
	InvalidCode {
		code : i32, message : String,
	},
	InvalidRequest(String),
}

impl fmt::Display for ErrType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			ErrType::InvalidURL(ref unk) => write!(f, "Could not properly form URL from the given settings: {}", unk),
			ErrType::InvalidSerde(ref unk) => write!(f, "Could not properly serde results: {}", unk),
			ErrType::InvalidCode {code, message} => write!(f, "Recieved an invalid status code {} after API call with message: \"{}\"", code, message),
			ErrType::InvalidRequest(ref unk) => write!(f, "Failed to make a request from the given settings: {}", unk),
		}
	}
}

impl From<ErrType> for Error {
    fn from(err_type: ErrType) -> Error {
        Error::from(Context::new(err_type))
    }
}

impl From<Context<ErrType>> for Error {
    fn from(context: Context<ErrType>) -> Error {
        Error { context }
    }
}

impl From<serde_json::Error> for Error {
	fn from(err : serde_json::Error) -> Self {
		Error::invalid_serde(err.to_string())
	}
}

impl From<reqwest::Error> for Error {
	fn from(err : reqwest::Error) -> Self {
		Error::invalid_request(err.to_string())
	}
}

impl From<reqwest::UrlError> for Error {
	fn from(err : reqwest::UrlError) -> Self {
		Error::invalid_url(err.to_string())
	}
}