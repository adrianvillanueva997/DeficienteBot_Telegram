use teloxide::{prelude::Requester, types::Message, Bot};

use crate::{
    message_checks::{reddit, tiktok, twitter},
    utils::format_message_username,
};

#[derive(Debug)]
pub struct SocialMediaHandler<'a> {
    bot: &'a Bot,
    msg: &'a Message,
}

impl<'a> SocialMediaHandler<'a> {
    pub fn new(bot: &'a Bot, msg: &'a Message) -> Self {
        SocialMediaHandler { bot, msg }
    }
    pub async fn process(
        &self,
        text: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if let Some(twitter) = twitter::update_twitter_links(text).await {
            self.format_social_media_message(&twitter).await?;
        } else if let Some(tiktok) = tiktok::updated_tiktok(text).await {
            self.format_social_media_message(&tiktok).await?;
        } else if let Some(reddit) = reddit::updated_reddit(text).await {
            self.format_social_media_message(&reddit).await?;
        }
        Ok(())
    }

    async fn format_social_media_message(
        &self,
        content: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let formatted = format_message_username(self.msg, content);
        self.bot
            .delete_message(self.msg.chat.id, self.msg.id)
            .await?;
        self.bot.send_message(self.msg.chat.id, formatted).await?;
        Ok(())
    }
}
