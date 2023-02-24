use tracing::*;


fn main() {
    println!("Hello, world!");

    let file_appender = tracing_appender::rolling::hourly("/home/bolean", "jankincai.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::fmt()
       .with_writer(non_blocking)
       .init();

    tracing::info!("jankincai");
}
