mod command;
mod cqbot;
mod gpt;
mod private_manager;
mod role;

use cqbot::Bot;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// cqhttp websocket address, default value is `ws://localhost:8080/ws`
    #[arg(short, long)]
    websocket: Option<String>,

    /// use proxy to access openai api, None to not use proxy
    #[arg(long)]
    proxy: Option<String>,

    /// openai api key, start with `sk-`
    api_key: String,

    /// bot qq, to determined if @ bot
    qq: u64,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let cqhttp_ws = args.websocket.unwrap_or("ws://localhost:8080/ws".into());

    let mut bot = Bot::new(&cqhttp_ws, args.proxy, args.api_key, args.qq).await;

    bot.loop_read().await;
}
