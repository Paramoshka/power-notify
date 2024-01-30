use teloxide::Bot;
use teloxide::repls::CommandReplExt;
use crate::r#mod::tg_bot::{answer, Command};

mod r#mod;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting command bot...");
    let bot = Bot::from_env();
    let bot = Bot::new();
    Command::repl(bot, answer).await;
    // Infinite loop
    loop {

    }
}