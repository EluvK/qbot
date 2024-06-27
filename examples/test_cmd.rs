use std::hint::black_box;

use clap::Parser;

fn main() {
    let args = qbot::qbot_cmd::QBotCmd::parse();
    black_box(args);
}
