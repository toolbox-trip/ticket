use std::fmt::Display;

use actix_web::http::StatusCode;
use error_chain::error_chain;

error_chain! {
    foreign_links {
        Reqwest(reqwest::Error);
        Url(url::ParseError);
        Jwt(jsonwebtoken::errors::Error);
        IoError(std::io::Error);
        StringFromUtf8(std::string::FromUtf8Error);
    }
    errors {
        CannotGetCache(msg: String) {
            description("Warn")
            display("Warn: {}", msg)
        }
    }
}

impl actix_web::ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self.0 {
            ErrorKind::Reqwest(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ErrorKind::Url(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ErrorKind::Jwt(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ErrorKind::IoError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ErrorKind::Msg(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ErrorKind::__Nonexhaustive {} => StatusCode::INTERNAL_SERVER_ERROR,
            ErrorKind::StringFromUtf8(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ErrorKind::CannotGetCache(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
