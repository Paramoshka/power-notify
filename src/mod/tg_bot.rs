use teloxide::Bot;
use teloxide::prelude::{Message, Requester, ResponseResult};
use teloxide::utils::command::BotCommands;
use crate::r#mod::json_config::read_config;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
pub enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "handle a username.")]
    Username(String),
    #[command(description = "handle a username and an age.", parse_with = "split")]
    UsernameAndAge { username: String, age: u8 },
}

pub async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    let admin_id = msg.from().unwrap().id.clone().to_string();
    let admin_id_from_file = read_config();

    if admin_id == admin_id_from_file.await.admin_id {
        match cmd {
            Command::Help => bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?,
            Command::Username(username) => {
                bot.send_message(msg.chat.id, format!("Your username is @{username}.")).await?
            }
            Command::UsernameAndAge { username, age } => {
                bot.send_message(msg.chat.id, format!("Your username is @{username} and age is {age}."))
                    .await?
            }
        };
    } else {
        bot.send_message(msg.chat.id, format!("Your id {admin_id} is not allowed")).await?;
    }



    Ok(())
}