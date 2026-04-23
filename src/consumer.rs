use serde::{Deserialize, Serialize};
use worker::{console_log, Context, Env, MessageBatch, MessageExt};
use worker_macros::event;

#[derive(Serialize, Debug, Clone, Deserialize)]
pub struct MyType {
    foo: String,
    bar: u32,
}

#[event(queue)]
async fn consume(
    message_batch: MessageBatch<MyType>,
    env: Env,
    _ctx: Context,
) -> worker::Result<()> {
    // Get a queue with the binding 'my_queue'
    let my_queue = env.queue("my_queue")?;

    // Deserialize the message batch
    let messages = message_batch.messages()?;

    // Loop through the messages
    for message in messages {
        // Log the message and meta data
        console_log!(
            "Got message {:?}, with id {} and timestamp: {}",
            message.body(),
            message.id(),
            message.timestamp().to_string()
        );

        // Send the message body to the other queue
        my_queue.send(message.body()).await?;

        // Ack individual message
        message.ack();

        // Retry individual message
        message.retry();
    }

    // Retry all messages
    message_batch.retry_all();
    // Ack all messages
    message_batch.ack_all();
    Ok(())
}
