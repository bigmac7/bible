use std::{error, fmt};

#[derive(Debug)]
pub enum AppError {
    Network(reqwest::Error),
    Database(rusqlite::Error),
    Io(std::io::Error),
    NotFound(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::Network(err) => {
                let url = err.url().map_or("the server".to_string(), |u| u.to_string());
                if err.is_connect() {
                    write!(f, "Could not connect to {}. Please check your internet connection and that the server is available.", url)
                } else if err.is_timeout() {
                    write!(f, "Connection to {} timed out. Please try again later.", url)
                } else if err.is_request() {
                    write!(f, "There was an issue sending the request to {}. This could be a DNS problem or a temporary network issue. Please check your connection.", url)
                } else if let Some(status) = err.status() {
                    match status {
                        reqwest::StatusCode::NOT_FOUND => write!(f, "The requested resource was not found at {}. Please check the name and try again.", url),
                        reqwest::StatusCode::FORBIDDEN | reqwest::StatusCode::UNAUTHORIZED => write!(f, "You are not authorized to access {}. Please check your credentials.", url),
                        s if s.is_server_error() => write!(f, "The server at {} encountered an error ({}). Please try again later.", url, s),
                        s if s.is_client_error() => write!(f, "There was a problem with the request to {} ({}). Please check your input.", url, s),
                        s => write!(f, "Received an unexpected response from {}: {}.", url, s),
                    }
                }
                else {
                    write!(f, "An unknown network error occurred when trying to reach {}.", url)
                }
            },
            AppError::Database(err) => write!(f, "A database error occurred: {}. If this persists, the database file may be corrupted.", err),
            AppError::Io(err) => write!(f, "A file system error occurred: {}. Please check file permissions and available disk space.", err),
            AppError::NotFound(item) => write!(f, "Could not find {}.", item),
        }
    }
}

impl error::Error for AppError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            AppError::Network(err) => Some(err),
            AppError::Database(err) => Some(err),
            AppError::Io(err) => Some(err),
            AppError::NotFound(_) => None,
        }
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> AppError {
        AppError::Network(err)
    }
}

impl From<rusqlite::Error> for AppError {
    fn from(err: rusqlite::Error) -> AppError {
        AppError::Database(err)
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> AppError {
        AppError::Io(err)
    }
}
