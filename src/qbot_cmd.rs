use clap::{Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
pub struct QBotCmd {
    #[command(subcommand)]
    pub sub: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// chat with bot
    #[command(subcommand)]
    Chat(ChatCommand),
    // config nps whitelist ip
    Nps {
        ip: String,
    },
}

#[derive(Debug, Subcommand)]
pub enum ChatCommand {
    /// clear chat history
    Clear,
    /// change chat mode
    Mode { mode: CommandChatMode },
    /// show information
    Info,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum CommandChatMode {
    Chat,
    Code,
}
