use crate::error::{try_handler, AppError};
use crate::service::user::{add, get, User};
use worker::{Context, Env, Request, Response, RouteContext, Router};
use worker_macros::event;

#[event(fetch)]
async fn fetch(req: Request, env: Env, _ctx: Context) -> worker::Result<Response> {
    tracing::info!(request=?req,"Received request");
    Router::new()
        .get_async("/user", user_get)
        .post_async("/user", user_add)
        .run(req, env)
        .await
}

async fn user_get(_req: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
    try_handler(async {
        let name = ctx
            .param("name")
            .ok_or_else(move || AppError::BadRequest("Name not be null".to_string()))?;
        get(name.clone(), ctx).await
    })
    .await
}
async fn user_add(mut req: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
    try_handler(async {
        let user: User = req
            .json()
            .await
            .map_err(|e| AppError::BadRequest(format!("Invalid Json: {e}")))?;
        add(user, ctx).await
    })
    .await
}
