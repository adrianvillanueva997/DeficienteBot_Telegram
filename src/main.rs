use frankenstein::AsyncTelegramApi;
use frankenstein::GetUpdatesParams;
use frankenstein::Message;
use frankenstein::SendMessageParams;
use frankenstein::{AsyncApi, UpdateContent};

use log::debug;
use log::error;
use log::info;

use pretty_env_logger::env_logger;
use std::env;

use deficientebot_telegram::checks::telegram_message::Checkings;
use deficientebot_telegram::typing_action;
#[tokio::main]
async fn main() {
    env_logger::init();
    let token = env::var("telegram_bot").expect("telegram_bot environment variable not found");
    let api = AsyncApi::new(&token);
    let update_params_builder = GetUpdatesParams::builder();
    let mut update_params = update_params_builder.clone().build();
    info!("Bot logged in");
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
                info!("Failed to get updates: {:?}", error);
            }
        }
    }
}

async fn process_message(message: Message, api: AsyncApi) {
    let message = Box::new(message);
    typing_action(&message, api).await;
    let message_content = &message.text.unwrap();
    let checkings = Checkings::build(String::from(message_content));

    if checkings.deficiente() {}
}
