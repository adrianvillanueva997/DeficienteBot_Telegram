use frankenstein::AsyncTelegramApi;
use frankenstein::GetUpdatesParams;
use frankenstein::Message;
use frankenstein::{AsyncApi, UpdateContent};

use log::{debug, error, info};

use pretty_env_logger::env_logger;
use std::env;

use deficientebot_telegram::checks::copypastas::copypasta;
use deficientebot_telegram::checks::telegram_message::Checkings;
use deficientebot_telegram::{
    delete_previous_message, happy_thursday_routine, send_message, send_reply_message,
    special_events, typing_action,
};
#[tokio::main]
async fn main() {
    env_logger::init();
    let token = env::var("telegram_bot").expect("telegram_bot environment variable not found");
    let api = AsyncApi::new(&token);
    let update_params_builder = GetUpdatesParams::builder();
    let mut update_params = update_params_builder.clone().build();
    info!("Bot logged in");

    happy_thursday_routine().await;
    special_events().await;
    loop {
        let result = api.get_updates(&update_params).await;
        debug!("result: {:?}", result);
        match result {
            Ok(response) => {
                for update in response.result {
                    if let UpdateContent::Message(message) = update.content {
                        let api_clone = api.clone();
                        tokio::spawn(async move {
                            process_message(message, api_clone).await;
                        });
                        update_params = update_params_builder
                            .clone()
                            .offset(update.update_id + 1)
                            .build();
                    }
                }
            }
            Err(error) => {
                error!("Failed to get updates: {:?}", error);
            }
        }
    }
}

async fn process_message(message: Message, api: AsyncApi) {
    let message_content = message.text.as_ref().unwrap();
    let checkings = Checkings::build(String::from(message_content));

    let vxtwitter_url = checkings.vx_twitter();
    if !vxtwitter_url.is_empty() {
        typing_action(&message, &api).await;
        let username = String::from(message.from.as_ref().unwrap().username.as_ref().unwrap());
        let space = String::from("\n");
        let at = String::from("@");
        let full_message = at + &username + &space + &vxtwitter_url;
        send_message(&message, &api, &full_message).await;
        delete_previous_message(&message, &api).await;
    }

    if checkings.deficiente() {
        typing_action(&message, &api).await;
        send_reply_message(&message, &api, "Deficiente").await;
    }

    if checkings.numerical_checks() {
        typing_action(&message, &api).await;
        send_reply_message(&message, &api, "> Nice").await;
    }
    let lowercase_message = message_content.to_lowercase();
    let copypasta_check = copypasta(&lowercase_message);
    if !copypasta_check.is_empty() {
        for copypasta in copypasta_check {
            send_reply_message(&message, &api, copypasta).await;
        }
    }
}
