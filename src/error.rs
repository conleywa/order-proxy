use std::future::Future;
use worker::Response;

pub enum AppError {
    BadRequest(String),
    NotFound(String),
    Internal(String),
}

impl AppError {
    pub fn into_response(self) -> worker::Result<Response> {
        let (msg, status) = match self {
            AppError::BadRequest(msg) => (msg, 400),
            AppError::NotFound(msg) => (msg, 404),
            AppError::Internal(msg) => (msg, 500),
        };
        Response::error(msg, status)
    }
}

impl From<worker::Error> for AppError {
    fn from(err: worker::Error) -> AppError {
        AppError::Internal(err.to_string())
    }
}

pub async fn try_handler<F>(f: F) -> worker::Result<Response>
where
    F: Future<Output = Result<Response, AppError>>,
{
    f.await.or_else(|e| e.into_response())
}
