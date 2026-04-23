use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;
use tracing_subscriber::fmt::format::Pretty;
use tracing_subscriber::fmt::time::UtcTime;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_web::{performance_layer, MakeConsoleWriter};
use worker_macros::event;

mod api;
mod consumer;
mod scheduler;

#[event(start)]
fn start() {
    let fmt_layer = tracing_subscriber::fmt::layer()
        .json()
        .with_ansi(false)
        .with_timer(UtcTime::rfc_3339())
        .with_writer(MakeConsoleWriter);
    let pref_layer = performance_layer().with_details_from_fields(Pretty::default());

    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(pref_layer)
        .init();
}

#[derive(Debug)]
pub enum AppError {
    NotFound,
    BadRequest(String),
    Unauthorized,
    Forbidden,
    Internal(String),
}

#[derive(Serialize)]
pub struct Err {
    pub msg: String,
}

impl From<worker::Error> for AppError {
    fn from(err: worker::Error) -> Self {
        AppError::Internal(err.to_string())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::NotFound => (
                StatusCode::NOT_FOUND,
                Json(Err {
                    msg: "not_found".into(),
                }),
            ),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, Json(Err { msg })),
            AppError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                Json(Err {
                    msg: "UNAUTHORIZED".into(),
                }),
            ),
            AppError::Forbidden => (
                StatusCode::FORBIDDEN,
                Json(Err {
                    msg: "FORBIDDEN".into(),
                }),
            ),
            AppError::Internal(_err) => (
                // log the err or put into a tracing span!
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(Err {
                    msg: "INTERNAL SERVER ERROR".into(),
                }),
            ),
        }
        .into_response()
    }
}
