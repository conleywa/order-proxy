use crate::error::{try_handler, AppError};
use serde::{Deserialize, Serialize};
use worker::{Context, Request, RouteContext};
use worker::{Date, Response};

#[derive(Serialize, Debug, Clone, Deserialize)]
pub struct User {
    pub name: String,
    pub birthday: u64,
    pub crate_at: u64,
}

pub async fn add(mut user: User, ctx: RouteContext<()>) -> Result<Response, AppError> {
    let queue = ctx.env.queue("demo_user_queue")?;
    user.crate_at = Date::now().as_millis();
    queue.send(user).await?;
    Ok(Response::ok("send success")?)
}

pub async fn get(username: String, ctx: RouteContext<()>) -> Result<Response, AppError> {
    let db = ctx.env.d1("demo_user_d1")?;
    let statement = db
        .prepare("SELECT name, birthday, create_at FROM t_user WHERE name = ?1")
        .bind(&[username.clone().into()])?;
    let result = statement.first::<User>(None).await?;
    match result {
        None => Err(AppError::NotFound(format!(
            "User with name {} not found",
            username
        ))),
        Some(user) => Ok(Response::from_json(&user)?),
    }
}
