use teloxide::Bot;
use teloxide::repls::CommandReplExt;
use crate::r#mod::json_config::read_config;
use crate::r#mod::tg_bot::{answer, Command};

mod r#mod;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting command bot...");
    let config = read_config().await;
    let bot = Bot::new(config.token);
    Command::repl(bot, answer).await;
    // Infinite loop
    loop {

    }
}