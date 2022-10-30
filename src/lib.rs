use frankenstein::{
    AsyncApi, AsyncTelegramApi, ChatAction, Message, SendChatActionParams, SendMessageParams,
};
use log::error;

pub mod checks;
mod tests;

pub async fn typing_action(message: &Message, api: AsyncApi) {
    let action_params = SendChatActionParams::builder()
        .action(ChatAction::Typing)
        .chat_id(message.chat.id)
        .build();
    if let Err(err) = api.send_chat_action(&action_params).await {
        error!("Failed to send chat action: {:?}", err);
    }
}

pub async fn send_reply_message(message: Message, api: AsyncApi) {
    let send_message_params = SendMessageParams::builder()
        .chat_id(message.chat.id)
        .text("deficiente")
        .reply_to_message_id(message.message_id)
        .build();
    if let Err(err) = api.send_message(&send_message_params).await {
        error!("Failed to send message: {:?}", err);
    }
}
