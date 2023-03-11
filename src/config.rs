use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    websocket: String,
    proxy: Option<String>,
    api_key: String,
    bot_qq: u64,
    root_qq: u64,
}

impl Config {
    #[cfg(test)]
    pub fn test_new() -> Self {
        Config {
            websocket: "ws://localhost:8080/ws".into(),
            proxy: Some("socks5h://127.0.0.1:1080".into()),
            api_key: "sk-xx".into(),
            bot_qq: 222,
            root_qq: 111,
        }
    }

    pub fn url(&self) -> &str {
        &self.websocket
    }
    pub fn proxy_addr(&self) -> &Option<String> {
        &self.proxy
    }
    pub fn api_key(&self) -> &str {
        &self.api_key
    }
    pub fn bot_qq(&self) -> u64 {
        self.bot_qq
    }
    pub fn root_qq(&self) -> u64 {
        self.root_qq
    }
}
