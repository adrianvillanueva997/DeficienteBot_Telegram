use std::{thread, time::Duration};

use frankenstein::{
    AsyncApi, AsyncTelegramApi, ChatAction, DeleteMessageParams, Message, SendChatActionParams,
    SendMessageParams,
};
use log::{error, info};
use routines::birthdays::special_event;

use crate::routines::utils::{calculate_next_midnight, is_thursday};
pub mod checks;
pub mod routines;
mod tests;

const GROUP_ID: &str = "-1063900471";

// const DEV_GROUP_ID: &str = "-281597102";

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

pub async fn send_message_to_group(api: AsyncApi, message_content: &str, group_id: &str) {
    let send_message_params = SendMessageParams::builder()
        .chat_id(String::from(group_id))
        .text(message_content)
        .build();
    if let Err(err) = api.send_message(&send_message_params).await {
        error!("Failed to send message: {:?}", err);
    }
}

pub async fn special_events(api: AsyncApi) {
    info!("Starting thursday routine");
    loop {
        info!("Calculating next midnight");
        let sleep_seconds = calculate_next_midnight();
        thread::sleep(Duration::from_secs(sleep_seconds));
        info!("Running special event routine");
        let event = special_event();
        if !event.is_empty() {
            send_message_to_group(api.to_owned(), event, GROUP_ID).await;
        }
    }
}

pub async fn happy_thursday_routine(api: AsyncApi) {
    info!("Starting thursday routine");
    let mut sent = false;
    loop {
        if is_thursday() && !sent {
            info!("Running thursday routine");
            send_message_to_group(api.to_owned(), "Feliz jueves!", GROUP_ID).await;
            sent = true;
        } else {
            info!("Calculating next midnight");
            let sleep_seconds = calculate_next_midnight();
            thread::sleep(Duration::from_secs(sleep_seconds));
            sent = false;
        }
    }
}
