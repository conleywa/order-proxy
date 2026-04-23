use crate::AppError;
use axum::body::Body;
use axum::extract::{Extension, State};
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use axum::{routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tower_service::Service;
use worker::{Context, Env, HttpRequest, Queue};
use worker_macros::event;

#[derive(Debug, Clone)]
struct AppState {
    queue: Arc<Queue>,
}

fn router(env: Env) -> Router {
    let queue = env
        .queue("test_queue")
        .expect("test_queue should be present");

    let app_state = AppState {
        queue: Arc::new(queue),
    };
    Router::new()
        .route("/", get(root))
        .route("/send", post(producer))
        .with_state(app_state)
}

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    env: Env,
    _ctx: Context,
) -> worker::Result<axum::http::Response<axum::body::Body>> {
    tracing::info!(request=?req,"Received request");
    Ok(router(env).call(req).await?)
}

pub async fn root() -> &'static str {
    "Hello Axum!"
}

#[derive(Serialize, Debug, Clone, Deserialize)]
pub struct Msg {
    title: String,
    text: String,
}

pub async fn producer(State(app_state): State<AppState>, Json(msg): Json<Msg>) -> Response {
    match app_state.queue.send(msg.clone()).await {
        Ok(_) => Response::new(Body::from("send success")),
        Err(e) => AppError::Internal(e.to_string()).into_response(),
    }
}
