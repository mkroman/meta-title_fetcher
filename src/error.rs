use rocket::http::{ContentType, Status};
use rocket::response::Responder;
use rocket::{response, Request, Response};
use std::io::{Cursor, Error as IoError};

#[derive(Debug, Fail)]
pub enum Error {
    /// A HTTP or connection error occurred.
    #[fail(display = "http client error")]
    ReqwestError {
        #[fail(cause)]
        error: reqwest::Error,
    },
    /// Parsing the given URI failed.
    #[fail(display = "URI parse error")]
    UriParseError,
    /// After parsing, no title could be found.
    #[fail(display = "No valid title found")]
    NoValidTitleError,
    /// The requested content is too big to parse.
    #[fail(display = "Content-Length exceeds limit: {}", _0)]
    ContentTooBigError(u64),
    /// The requested content does not have a defined content-length.
    #[fail(display = "Content-Length is not returned")]
    ContentLengthMissingError,
    /// There was an I/O error.
    #[fail(display = "I/O error: {}", error)]
    IoError {
        #[fail(cause)]
        error: IoError,
    },
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Error {
        Error::ReqwestError { error }
    }
}

impl From<IoError> for Error {
    fn from(error: IoError) -> Error {
        Error::IoError { error }
    }
}

impl<'a> Responder<'a> for Error {
    fn respond_to(self, _: &Request) -> response::Result<'a> {
        Response::build()
            .header(ContentType::JSON)
            .status(Status::InternalServerError)
            .sized_body(Cursor::new(format!("{}", self)))
            .ok()
    }
}
