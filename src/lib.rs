use tracing_subscriber::fmt::format::Pretty;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_web::{performance_layer, MakeConsoleWriter};
use worker_macros::event;

mod api;
mod consumer;
mod error;
mod scheduler;
mod service;

#[event(start)]
fn start() {
    console_error_panic_hook::set_once();

    let fmt_layer = tracing_subscriber::fmt::layer()
        .json()
        .with_ansi(false)
        .without_time()
        .with_writer(MakeConsoleWriter);
    let perf_layer = performance_layer().with_details_from_fields(Pretty::default());

    let _ = tracing_subscriber::registry()
        .with(fmt_layer)
        .with(perf_layer)
        .try_init();
}
