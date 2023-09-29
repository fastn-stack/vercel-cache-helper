extern crate self as vercel_cache_helper;

use std::convert::From;

#[derive(Debug)]
pub enum Error {
    FileNotFound(String),
    InvalidInput(String),
    EnvVarNotFound(String),
    ReqwestError(String),
    Other(Box<dyn std::error::Error + Send + Sync>),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::FileNotFound(message) => write!(f, "File not found: {}", message),
            Error::InvalidInput(message) => write!(f, "Invalid input: {}", message),
            Error::EnvVarNotFound(var_name) => {
                write!(f, "Environment variable not found: {}", var_name)
            }
            Error::ReqwestError(message) => write!(f, "Reqwest error: {}", message),
            Error::Other(inner) => write!(f, "Other error: {}", inner),
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;

impl From<std::io::Error> for Error {
    fn from(io_error: std::io::Error) -> Self {
        Error::FileNotFound(io_error.to_string())
    }
}

impl From<std::env::VarError> for Error {
    fn from(env_error: std::env::VarError) -> Self {
        Error::EnvVarNotFound(env_error.to_string())
    }
}

impl From<reqwest::Error> for Error {
    fn from(reqwest_error: reqwest::Error) -> Self {
        Error::ReqwestError(reqwest_error.to_string())
    }
}

pub mod commands;
pub mod utils;
pub mod vercel;

pub fn get_remote_client(
    token: String,
    team_id: Option<String>,
    product: String,
) -> vercel::remote_client::RemoteClient {
    vercel::remote_client::RemoteClient::new(token, team_id, product)
}
