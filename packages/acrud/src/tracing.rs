use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn init() {
    let json_format = tracing_subscriber::fmt::format::Format::default().json();

    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_ansi(false)
        .event_format(json_format);

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "tower_http=debug".into()),
        ))
        .with(fmt_layer)
        .init();
}
