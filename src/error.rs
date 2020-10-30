use error_chain::error_chain;
use actix_web::http::StatusCode;

error_chain! {
    foreign_links {
        Reqwest(reqwest::Error);
        Url(url::ParseError);
        Jwt(jsonwebtoken::errors::Error);
    }
}

impl actix_web::ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self.0 {
            ErrorKind::Reqwest(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            ErrorKind::Url(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            ErrorKind::Jwt(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            ErrorKind::Msg(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            ErrorKind::__Nonexhaustive {  } => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }
}
