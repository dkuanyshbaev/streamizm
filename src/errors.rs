use rocket::http::Status;
use rocket::request::Request;
use rocket::response::Responder;
use std::convert::From;
use std::{error, fmt};

#[derive(Debug)]
pub enum SeasonsError {
    NotFound,
    InternalServerError,
}

pub type SeasonsResult<T> = Result<T, SeasonsError>;

impl fmt::Display for SeasonsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SeasonsError::NotFound => write!(f, "NotFound"),
            SeasonsError::InternalServerError => write!(f, "InternalServerError"),
        }
    }
}

impl error::Error for SeasonsError {
    fn description(&self) -> &str {
        match *self {
            SeasonsError::NotFound => "Record not found",
            SeasonsError::InternalServerError => "Internal server error",
        }
    }
}
