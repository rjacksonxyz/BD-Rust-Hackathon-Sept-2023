use actix_web::{http::StatusCode, HttpResponse, ResponseError};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("this resource requires higher privileges")]
    Forbidden,
    #[error("this resource was not found")]
    NotFound,
    #[error("an unhandled database error occurred")]
    DatabaseError(#[from] sqlx::Error),
    #[error("an unspecified internal error occurred: {0}")]
    InternalError(#[from] anyhow::Error),
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match &self {
            Self::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Forbidden => StatusCode::FORBIDDEN,
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).body(self.to_string())
    }
}

// Short hand alias, which allows you to use just Result<T>
pub type Result<T> = std::result::Result<T, Error>;
