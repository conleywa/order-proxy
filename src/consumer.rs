use crate::service::user::User;
use worker::{Context, Env, MessageBatch, MessageExt};
use worker_macros::event;

#[event(queue)]
async fn consume(message_batch: MessageBatch<User>, env: Env, _ctx: Context) -> worker::Result<()> {
    // Deserialize the message batch
    let messages = message_batch.messages()?;
    // Loop through the messages
    for message in messages {
        tracing::info!(
            "Got message {:?}, with id {} and timestamp: {}",
            message.body(),
            message.id(),
            message.timestamp().to_string()
        );
        let db = env.d1("demo_user_d1")?;
        let statement =
            db.prepare("INSERT INTO t_user (name, birthday, created_at) VALUES (?1, ?2, ?3)");
        let user = message.body();
        let result = statement
            .bind(&[
                user.name.clone().into(),
                user.birthday.into(),
                user.crate_at.into(),
            ])?
            .run()
            .await?;
        tracing::info!("INSERT result: {:?}, user={:?}", result, user);
    }
    message_batch.ack_all();
    Ok(())
}
