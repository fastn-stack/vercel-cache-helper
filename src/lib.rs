extern crate self as vercel_cache_helper;

use std::convert::From; // Import From trait for conversion

#[derive(Debug)]
pub enum Error {
    FileNotFound(String),
    InvalidInput(String),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::FileNotFound(message) => write!(f, "File not found: {}", message),
            Error::InvalidInput(message) => write!(f, "Invalid input: {}", message),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

// Implement the From trait for conversion to your custom error type
impl From<std::io::Error> for Error {
    fn from(io_error: std::io::Error) -> Self {
        Error::FileNotFound(io_error.to_string())
    }
}

impl From<reqwest::Error> for Error {
    fn from(reqwest_error: reqwest::Error) -> Self {
        Error::InvalidInput(reqwest_error.to_string())
    }
}

mod vercel;
mod commands;
