use std::convert::Infallible;
use std::time::Duration;

use log::error;
use message_checks::{bad_words, thursday, webm};
use teloxide::net::Download;
use teloxide::payloads::{SendAudioSetters, SendMessageSetters, SendVideoSetters};
use teloxide::requests::Requester;
use teloxide::types::{Document, Message};
use teloxide::update_listeners::UpdateListener;
use teloxide::Bot;
use tokio::fs;
use tokio::time::sleep;

mod message_checks;

/// Bot logic goes here.
///
/// # Panics
///
/// Panics if the bot fails to run.
///
/// # Errors
///
/// This function will return an error if the bot fails to run.
async fn process_text_messages(
    bot: &Bot,
    msg: &Message,
    text: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let message = text.to_lowercase();
    let mut actions: Vec<_> = Vec::new();

    if message_checks::url::is_url(&message) {
        let twitter = message_checks::twitter::update_vxtwitter(&message).await;
        if let Some(twitter) = twitter {
            let message = msg.clone();
            let user = message.from().as_ref().unwrap().username.as_ref().unwrap();
            let tweet = format!("@{} \n{} ", user, twitter);
            bot.delete_message(msg.chat.id, msg.id).await?;
            actions.push(bot.send_message(msg.chat.id, tweet));
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
                .reply_to_message_id(msg.id)
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
        if copypasta == "viernes" {
            bot.send_chat_action(msg.chat.id, teloxide::types::ChatAction::RecordVoice)
                .await?;
            bot.send_audio(
                msg.chat.id,
                teloxide::types::InputFile::file(std::path::Path::new("viernes.ogg")),
            )
            .reply_to_message_id(msg.id)
            .await?;
        } else {
            actions.push(
                bot.send_message(msg.chat.id, copypasta)
                    .reply_to_message_id(msg.id),
            );
        }
    }

    if thursday::is_thursday().await && thursday::check_asuka(&message).await {
        actions.push(
            bot.send_message(msg.chat.id, thursday::random_message().await)
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
    Ok(())
}

/// Checks if the file is a webm and converts it to mp4 and sends it.
///
/// # Panics
///
/// Panics if the bot fails to download the file or convert it.
///
/// # Errors
///
/// This function will return an error if the bot fails to download the file or convert it.
pub async fn process_files(
    bot: &Bot,
    msg: &Message,
    _file: &Document,
) -> Result<(), Box<dyn std::error::Error>> {
    if _file.clone().file_name.unwrap().contains("webm") && _file.clone().file.size <= 20000000 {
        // 20 MB
        if webm::webm_exists().await {
            webm::delete_webm().await;
        }
        if webm::mp4_exists().await {
            webm::delete_mp4().await;
        }
        bot.send_chat_action(msg.chat.id, teloxide::types::ChatAction::UploadVideo)
            .await
            .unwrap();
        let file = bot.get_file(_file.file.clone().id).await.unwrap();
        let mut dst = fs::File::create(webm::WEBM).await.unwrap();
        bot.download_file(&file.path, &mut dst).await.unwrap();
        webm::convert_webm_to_mp4().await;
        bot.send_video(
            msg.chat.id,
            teloxide::types::InputFile::file(std::path::Path::new(webm::MP4)),
        )
        .reply_to_message_id(msg.id)
        .await?;
    }
    Ok(())
}

/// Handles messages from the bot.
///
/// # Errors
///
/// This function will return an error if .
pub async fn handle_messages(bot: &Bot, msg: &Message) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(text) = msg.text() {
        process_text_messages(bot, msg, text).await?
    } else if let Some(file) = msg.document() {
        process_files(bot, msg, file).await?
    }
    Ok(())
}

/// Parse messages from the bot.
pub async fn parse_messages(bot: Bot, listener: impl UpdateListener<Err = Infallible> + Send) {
    teloxide::repl_with_listener(
        bot,
        |bot, msg| async move {
            if let Err(err) = handle_messages(&bot, &msg).await {
                error!("Error processing text messages: {}", err);
            }
            Ok(())
        },
        listener,
    )
    .await;
}
