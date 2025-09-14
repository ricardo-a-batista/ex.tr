use tracing_subscriber::{prelude::*, util::SubscriberInitExt};

pub fn init() {
    let console_layer = console_subscriber::ConsoleLayer::builder().spawn();
    let fmt_layer = tracing_subscriber::fmt::layer();

    tracing_subscriber::registry()
        .with(console_layer)
        .with(fmt_layer)
        .try_init()
        .ok();
}
