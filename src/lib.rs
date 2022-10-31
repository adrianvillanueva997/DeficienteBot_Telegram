use frankenstein::{
    AsyncApi, AsyncTelegramApi, ChatAction, DeleteMessageParams, Message, SendChatActionParams,
    SendMessageParams,
};
use log::error;

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
    let send_message_params = SendMessageParams::builder()
        .chat_id(message.chat.id)
        .text(message_content)
        .build();
    if let Err(err) = api.send_message(&send_message_params).await {
        error!("Failed to delete message: {:?}", err);
    }
}
