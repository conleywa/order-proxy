use worker::{ScheduleContext, ScheduledEvent};
use worker_macros::event;

#[event(scheduled)]
pub async fn scheduled(event: ScheduledEvent, env: worker::Env, _ctx: ScheduleContext) {
    tracing::info!(?event, "Scheduled event");
    let db = env.d1("demo_user_d1").unwrap();
    let result = db
        .prepare("select count(*) from t_user")
        .first::<u64>(None)
        .await
        .unwrap()
        .unwrap_or(0);

    tracing::info!(?result, "Current user count={}", result);
}
