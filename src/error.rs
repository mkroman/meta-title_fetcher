use rocket::http::{ContentType, Status};
use rocket::response::Responder;
use rocket::{response, Request, Response};
use thiserror::Error;

use std::io::{Cursor, Error as IoError};

#[derive(Debug, Error)]
pub enum Error {
    /// A HTTP or connection error occurred.
    #[error("http client error")]
    ReqwestError(#[from] reqwest::Error),
    /// Parsing the given URI failed.
    #[error("URL parse error")]
    UriParseError,
    /// After parsing, no title could be found.
    #[error("No valid title found")]
    NoValidTitleError,
    /// The requested content is too big to parse.
    #[error("Content-Length exceeds limit: {0}")]
    ContentTooBigError(u64),
    /// There was a problem reading the config file.
    #[error("failed to parse config")]
    ConfigError(#[from] toml::de::Error),
    /// The requested content does not have a defined content-length.
    #[error("Content-Length is not returned")]
    ContentLengthMissingError,
    /// There was an I/O error.
    #[error("I/O error")]
    IoError(#[from] IoError),
}

// impl From<reqwest::Error> for Error {
//     fn from(error: reqwest::Error) -> Error {
//         Error::ReqwestError { error }
//     }
// }
//
// impl From<IoError> for Error {
//     fn from(error: IoError) -> Error {
//         Error::IoError { error }
//     }
// }
//
// impl From<toml::de::Error> for Error {
//     fn from(error: toml::de::Error) -> Error {
//         Error::ConfigError { error }
//     }
// }

impl<'a> Responder<'a> for Error {
    fn respond_to(self, _: &Request) -> response::Result<'a> {
        Response::build()
            .header(ContentType::JSON)
            .status(Status::InternalServerError)
            .sized_body(Cursor::new(format!("{}", self)))
            .ok()
    }
}
