use std::{env, process::Command};

use chrono::Weekday;
use frankenstein::{
    AsyncApi, AsyncTelegramApi, ChatAction, DeleteMessageParams, Message, SendChatActionParams,
    SendMessageParams,
};
use log::{error, info};
use routines::birthdays::special_event;
use tokio::spawn;
use tokio_schedule::{every, Job};

pub mod checks;
pub mod routines;
mod tests;

pub async fn typing_action(message: &Message, api: &AsyncApi) {
    let action_params = SendChatActionParams::builder()
        .action(ChatAction::Typing)
        .chat_id(message.chat.id)
        .build();
    if let Err(err) = api.send_chat_action(&action_params).await {
        error!("Failed to send chat action: {:?}", err);
    }
}

pub async fn send_reply_message(message: &Message, api: &AsyncApi, message_content: &str) {
    typing_action(message, api).await;
    let send_message_params = SendMessageParams::builder()
        .chat_id(message.chat.id)
        .text(message_content)
        .reply_to_message_id(message.message_id)
        .build();
    if let Err(err) = api.send_message(&send_message_params).await {
        error!("Failed to send message: {:?}", err);
    }
}

pub async fn delete_previous_message(message: &Message, api: &AsyncApi) {
    let delete_message_params = DeleteMessageParams::builder()
        .chat_id(message.chat.id)
        .message_id(message.message_id)
        .build();
    if let Err(err) = api.delete_message(&delete_message_params).await {
        error!("Failed to delete message: {:?}", err);
    }
}

pub async fn send_message(message: &Message, api: &AsyncApi, message_content: &str) {
    typing_action(message, api).await;
    let send_message_params = SendMessageParams::builder()
        .chat_id(message.chat.id)
        .text(message_content)
        .build();
    if let Err(err) = api.send_message(&send_message_params).await {
        error!("Failed to delete message: {:?}", err);
    }
}

pub async fn happy_thursday_routine() {
    info!("Setting up thursday routine");
    let thursday_routine = every(1)
        .week()
        .on(Weekday::Thu)
        .at(00, 00, 00)
        .perform(|| async {
            info!("Running task");
            let telegram_url = format!(
                "https://api.telegram.org/bot{}/sendMessage",
                env::var("telegram_bot").expect("Environment key not found")
            );
            let _ = Command::new("http")
                .args([
                    "POST",
                    &telegram_url,
                    "chat_id=-1001063900471",
                    "text=Feliz jueves!",
                ])
                .spawn();
        });
    spawn(thursday_routine);
}

pub async fn special_events() {
    info!("Setting up special events routine");
    let every_day_routine = every(1).day().at(00, 00, 00).perform(|| async {
        info!("Running task");
        let event = special_event();
        if !event.is_empty() {
            let telegram_url = format!(
                "https://api.telegram.org/bot{}/sendMessage",
                env::var("telegram_bot").expect("Environment key not found")
            );
            let message = format!("text={}", event);
            let _ = Command::new("http")
                .args(["POST", &telegram_url, "chat_id=-1001063900471", &message])
                .spawn();
        }
    });
    spawn(every_day_routine);
}
