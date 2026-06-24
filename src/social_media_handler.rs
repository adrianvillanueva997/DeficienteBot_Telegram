use teloxide::{prelude::Requester, types::Message, Bot};

use crate::{message_checks::{reddit, tiktok, twitter}, utils::format_message_username};

pub async fn process(
    bot: &Bot,
    msg: &Message,
    text: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let content = if let Some(twitter) = twitter::update_twitter_links(text).await {
        Some(twitter)
    } else if let Some(tiktok) = tiktok::updated_tiktok(text).await {
        Some(tiktok)
    } else {
        reddit::updated_reddit(text).await
    };

    if let Some(content) = content {
        let formatted = format_message_username(msg, &content);
        bot.delete_message(msg.chat.id, msg.id).await?;
        bot.send_message(msg.chat.id, formatted).await?;
    }
    Ok(())
}
