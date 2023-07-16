use std::convert::Infallible;
use std::time::Duration;

use log::error;
use message_checks::{bad_words, thursday, webm};
use teloxide::payloads::SendMessageSetters;
use teloxide::requests::Requester;
use teloxide::types::Message;
use teloxide::update_listeners::UpdateListener;
use teloxide::Bot;
use tokio::time::sleep;

mod message_checks;

async fn process_text_messages(bot: &Bot, msg: &Message) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(text) = msg.text() {
        let message = text.to_lowercase();
        let mut actions: Vec<_> = Vec::new();

        if message_checks::url::is_url(&message) {
            let twitter = message_checks::twitter::update_vxtwitter(&message).await;
            if let Some(twitter) = twitter {
                bot.delete_message(msg.chat.id, msg.id).await?;
                actions.push(bot.send_message(msg.chat.id, twitter));
            } else if message_checks::webm::url_is_webm(&message) {
                if message_checks::webm::check_url_status_code(&message).await != Some(200) {
                    actions.push(
                        bot.send_message(msg.chat.id, "El video no existe :(")
                            .reply_to_message_id(msg.id),
                    );
                } else {
                    if webm::webm_exists().await {
                        webm::delete_webm().await;
                    }
                    if webm::mp4_exists().await {
                        webm::delete_mp4().await;
                    }
                    bot.send_chat_action(msg.chat.id, teloxide::types::ChatAction::UploadVideo)
                        .await?;
                    message_checks::webm::download_webm(&message).await;
                    message_checks::webm::convert_webm_to_mp4().await;
                    bot.send_video(
                        msg.chat.id,
                        teloxide::types::InputFile::file(std::path::Path::new(
                            message_checks::webm::MP4,
                        )),
                    )
                    .await?;
                }
            }
        }

        if bad_words::find_bad_words(&message).await {
            actions.push(
                bot.send_message(msg.chat.id, "Deficiente")
                    .reply_to_message_id(msg.id),
            );
        }

        let copypastas = message_checks::copypasta::find_copypasta(&message);
        for copypasta in copypastas.await {
            actions.push(
                bot.send_message(msg.chat.id, copypasta)
                    .reply_to_message_id(msg.id),
            );
        }

        if thursday::is_thursday().await && thursday::check_asuka(&message).await {
            actions.push(
                bot.send_message(msg.chat.id, &thursday::random_message().await)
                    .reply_to_message_id(msg.id),
            );
        }

        if !actions.is_empty() {
            bot.send_chat_action(msg.chat.id, teloxide::types::ChatAction::Typing)
                .await?;
            tokio::join!(async {
                for action in actions {
                    action.await.unwrap();
                    sleep(Duration::from_secs(1)).await;
                }
            });
        }
    }

    Ok(())
}

pub async fn parse_messages(bot: Bot, listener: impl UpdateListener<Err = Infallible> + Send) {
    teloxide::repl_with_listener(
        bot,
        |bot, msg| async move {
            if let Err(err) = process_text_messages(&bot, &msg).await {
                error!("Error processing text messages: {}", err);
            }
            Ok(())
        },
        listener,
    )
    .await;
}
