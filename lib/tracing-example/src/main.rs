use tracing_subscriber::prelude::*;


fn main() {
    // simple example
    // tracing_subscriber::fmt::init();

    // custom example

    // support RUST_LOG=DEBUG cargo run
    // support RUST_LOG=my_crate=DEBUG cargo run
    // support RUST_LOG=my_crate::xx=DEBUG cargo run
    // support line_number
    // support thread_ids
    // support write file
    let builder = tracing_subscriber::fmt::Subscriber::builder();
    let builder = builder.with_env_filter(tracing_subscriber::EnvFilter::from_default_env());

    let file_appender = tracing_appender::rolling::daily(".", "jankincai.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    let builder = builder
        .with_writer(non_blocking)
        // .with_max_level(tracing_subscriber::filter::LevelFilter::TRACE)
        .with_line_number(true)
        .with_thread_ids(true);
    let subscriber = builder.finish();

    let subscriber = {
        use tracing_subscriber::{filter::Targets, layer::SubscriberExt, fmt::Subscriber};
        use std::{env, str::FromStr};
        let targets = match env::var("RUST_LOG") {
            Ok(var) => Targets::from_str(&var)
                .map_err(|e| {
                    eprintln!("Ignoring `RUST_LOG={:?}`: {}", var, e);
                })
                .unwrap_or_default(),
            Err(env::VarError::NotPresent) => {
                Targets::new().with_default(Subscriber::DEFAULT_MAX_LEVEL)
            }
            Err(e) => {
                eprintln!("Ignoring `RUST_LOG`: {}", e);
                Targets::new().with_default(Subscriber::DEFAULT_MAX_LEVEL)
            }
        };
        subscriber.with(targets)
    };

    subscriber.init();

    tracing::error!("jankincai");
    tracing::warn!("jankincai");
    tracing::info!("jankincai");
    tracing::debug!("jankincai");
    tracing::trace!("jankincai");
}
