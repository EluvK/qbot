use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct QBotCmd {
    #[command(subcommand)]
    pub sub: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// manager server
    Server {
        // #[clap(short, long)]
        // list: bool,
        /// query Server status
        #[clap(long, value_name = "Save Name")]
        status: Option<String>,

        // #[clap(short, long, value_name = "Save Name")]
        // new: Option<String>,
        /// start Server with Save
        #[clap(long, value_name = "Save Name")]
        start: Option<String>,

        /// stop Server with Save
        #[clap(long, value_name = "Save Name")]
        stop: Option<String>,

        /// backup current file while running
        #[clap(long, value_name = "Save Name")]
        save: Option<String>,
    },
    Config {
        r#type: String,
    },
    Nps {
        ip: String,
    },
    // Info {
    //     #[clap(short, long)]
    //     query: Option<String>
    // }
}
