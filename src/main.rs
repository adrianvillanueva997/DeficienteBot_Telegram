use deficientebot_telegram::{
    checks::{copypastas::copypasta, telegram_message::Checkings},
    happy_thursday_routine, special_events,
};

use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting bot");
    let bot = Bot::from_env();
    log::info!("Bot logged in successfuly");
    let _ = tokio::spawn(special_events(bot.to_owned()));
    let _ = tokio::spawn(happy_thursday_routine(bot.to_owned()));
    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        let message = msg.text();
        if let Some(message) = message {
            let checkings = Checkings::build(String::from(message));
            let vxtwitter_url = checkings.vx_twitter();
            if !vxtwitter_url.is_empty() {
                let username =
                    String::from(msg.from().as_ref().unwrap().username.as_ref().unwrap());
                let space = String::from("\n");
                let at = String::from("@");
                let full_message = at + &username + &space + &vxtwitter_url;
                let _ = bot
                    .send_message(msg.chat.id, full_message)
                    .reply_to_message_id(msg.id)
                    .await;
            }

            if checkings.deficiente() {
                let _ = bot
                    .send_message(msg.chat.id, "Deficiente")
                    .reply_to_message_id(msg.id)
                    .await;
            }

            if checkings.numerical_checks() {
                let _ = bot
                    .send_message(msg.chat.id, "> Nice")
                    .reply_to_message_id(msg.id)
                    .await;
            }

            let lowercase_message = message.to_lowercase();
            let copypasta_check = copypasta(&lowercase_message);
            if !copypasta_check.is_empty() {
                for copypasta in copypasta_check {
                    let _ = bot
                        .send_message(msg.chat.id, copypasta)
                        .reply_to_message_id(msg.id)
                        .await;
                }
            }
        }

        Ok(())
    })
    .await;
}
