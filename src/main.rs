mod chat;
mod config;
mod qbot;
mod qbot_cmd;

use std::path::Path;

use clap::Parser;

use crate::qbot::QBot;

#[derive(Parser)]
struct Args {
    #[clap(short, long)]
    config: Option<String>,

    /// init config file in path
    #[clap(long)]
    init_config: bool,

    /// log file path
    #[clap(long)]
    log_path: Option<String>,

    /// enable debug log
    #[clap(long)]
    debug: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    if args.init_config {
        std::fs::write(Path::new("./config.yaml"), config::default_config())?;
        return Ok(());
    }
    let config_path_str = args.config.unwrap_or("./config.yaml".into());
    let config_path = Path::new(&config_path_str);
    let config = config::load_from_file(config_path)?;
    println!("config: {config:?}");
    let log_path_str = args.log_path.unwrap_or("./".into());
    let log_path = Path::new(&log_path_str);

    let _g = file_log(log_path, args.debug)?;
    println!("---- start QBot ----");
    let qbot = QBot::new(config).await;
    qbot.start().await;

    Ok(())
}

fn file_log(path: &Path, enable_debug: bool) -> anyhow::Result<impl Drop> {
    let file_path = path.join("logs");
    println!("logs file to: {file_path:?}");
    let file_appender = tracing_appender::rolling::RollingFileAppender::builder()
        .rotation(tracing_appender::rolling::Rotation::DAILY)
        .filename_prefix("qbot")
        .filename_suffix("log")
        .build(file_path)?;
    let (non_blocking_appender, guard) = tracing_appender::non_blocking(file_appender);
    let mut subscriber = tracing_subscriber::fmt()
        .with_writer(non_blocking_appender)
        .with_timer(tracing_subscriber::fmt::time::UtcTime::rfc_3339())
        .with_ansi(false);
    if enable_debug {
        subscriber = subscriber.with_max_level(tracing::Level::DEBUG);
    }
    tracing::subscriber::set_global_default(subscriber.finish()).unwrap();
    tracing::info!("start");

    Ok(guard)
}
