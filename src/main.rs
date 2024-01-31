use teloxide::Bot;
use teloxide::repls::CommandReplExt;
use crate::r#mod::json_config::read_config;
use crate::r#mod::tg_bot::{answer, check_ethernet, Command, send_on_start};

mod r#mod;

#[tokio::main]
async fn main() {
    loop {
       let check =  check_ethernet().await;
        log::info!("checking internet connection with https://api.telegram.org");
        match check {
            Ok(_) => {
                break;
            }
            Err(e) => println!("{e}")
        }

    }
    pretty_env_logger::init();
    log::info!("Starting command bot...");
    let config = read_config().await;
    let bot = Bot::new(config.token);
    send_on_start(bot.clone()).await.expect("Error send message, maybe chat id not set");
    Command::repl(bot, answer).await;
}