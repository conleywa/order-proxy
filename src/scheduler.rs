use worker::{ScheduleContext, ScheduledEvent};
use worker_macros::event;

#[event(scheduled)]
pub async fn scheduled(event: ScheduledEvent, _env: worker::Env, _ctx: ScheduleContext) {
    tracing::info!(?event, "Scheduled event");
}
