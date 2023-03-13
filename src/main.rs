mod command;
mod config;
mod cqbot;
mod gpt;
mod history;
mod private_manager;
mod role;

use config::Config;
use cqbot::Bot;

use std::{fs::File, io::Read, path::Path};

#[tokio::main]
async fn main() {
    let path = Path::new("config.json");

    if !path.exists() {
        let mut _file = File::create(path).expect("Failed to create config file");
        std::fs::write(path, r#"{"websocket": "ws://localhost:8080/ws", "proxy": "", "api_key": "sk-xxx", "bot_qq": 123, "root_qq": 456}"#).expect("Failed to create config file");
        println!("Config file created, you should edit it and run again");
        return;
    }

    let mut file = File::open("config.json").expect("Failed to open config file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file content");

    let config = serde_json::from_str::<Config>(&contents).expect("Check config file content");

    let mut bot = Bot::new(config).await;

    bot.loop_read().await;
}
