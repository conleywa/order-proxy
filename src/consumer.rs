use crate::service::user::User;
use worker::wasm_bindgen::JsValue;
use worker::{Context, Env, MessageBatch, MessageExt};
use worker_macros::event;

#[event(queue)]
async fn consume(message_batch: MessageBatch<User>, env: Env, _ctx: Context) -> worker::Result<()> {
    let messages = message_batch.messages()?;
    let db = env.d1("demo_user_d1")?;

    for message in messages {
        tracing::info!(
            "Got message {:?}, with id {} and timestamp: {}",
            message.body(),
            message.id(),
            message.timestamp().to_string()
        );
        let user = message.body();
        let result = db
            .prepare("INSERT INTO t_user (name, birthday, created_at) VALUES (?1, ?2, ?3)")
            .bind(&[
                user.name.clone().into(),
                JsValue::from_f64(user.birthday as f64),
                JsValue::from_f64(user.created_at as f64),
            ])?
            .run()
            .await?;
        tracing::info!("INSERT result: {:?}, user={:?}", result, user);
    }
    message_batch.ack_all();
    Ok(())
}
