use std::convert::Infallible;

use std::time::Duration;

use message_checks::{bad_words, thursday, webm};
use ranking::rank::Rank;
use tracing::{error, instrument};
use uuid::Uuid;

use std::error::Error;
use teloxide::net::Download;
use teloxide::payloads::{SendAudioSetters, SendMessageSetters, SendVideoSetters};
use teloxide::requests::Requester;
use teloxide::types::{Document, Message};
use teloxide::update_listeners::UpdateListener;
use teloxide::Bot;
use tokio::fs;
use tokio::time::sleep;

pub mod message_checks;
pub mod ranking;
pub mod redis_connection;

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
    redis_connection: &redis::Client,
    text: &str,
) -> Result<(), Box<dyn Error>> {
    let message = text.to_lowercase();
    let mut actions: Vec<_> = Vec::new();
    if message_checks::url::is_url(&message) {
        let twitter = message_checks::twitter::update_vxtwitter(&message).await;
        if let Some(twitter) = twitter {
            let message = msg.clone();
            let user = message.from().as_ref().unwrap().username.as_ref().unwrap();
            let tweet = format!("@{} \n{} ", user, twitter);
            bot.delete_message(msg.chat.id, msg.id).await?;
            Rank::new(redis_connection.clone())
                .update_rank("twitter".to_string())
                .await;
            actions.push(bot.send_message(msg.chat.id, tweet));
        } else if webm::url_is_webm(&message) {
            if webm::check_url_status_code(&message).await != Some(200) {
                actions.push(
                    bot.send_message(msg.chat.id, "El video no existe 😭")
                        .reply_to_message_id(msg.id),
                );
            } else {
                // webm::files_exist().await; // TODO: Instead of checking this, do clenaup after sending the video.
                let uuid = Uuid::new_v4();
                let webm_filename = format!("{}.webm,", uuid);
                let mp4_filename = format!("{}.mp4", uuid);
                bot.send_chat_action(msg.chat.id, teloxide::types::ChatAction::UploadVideo)
                    .await?;
                webm::download_webm(&message, &webm_filename).await;
                webm::convert_webm_to_mp4(&webm_filename, &mp4_filename).await;
                bot.send_video(
                    msg.chat.id,
                    teloxide::types::InputFile::file(std::path::Path::new(&mp4_filename)),
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
#[instrument]
pub async fn process_files(
    bot: &Bot,
    msg: &Message,
    redis_connection: &redis::Client,
    _file: &Document,
) -> Result<(), Box<dyn Error>> {
    // Telegram max file size: 20 MB
    if _file.clone().file_name.unwrap().contains("webm") && _file.clone().file.size <= 20000000 {
        // webm::files_exist().await; // TODO: Instead of checking this, do clenaup after sending the video.
        let uuid = Uuid::new_v4();
        let webm_filename = format!("{}.webm,", uuid);
        let mp4_filename = format!("{}.mp4", uuid);
        bot.send_chat_action(msg.chat.id, teloxide::types::ChatAction::UploadVideo)
            .await
            .unwrap();
        let file = bot.get_file(_file.file.clone().id).await.unwrap();
        let mut dst = fs::File::create(format!("{}.webm", webm_filename))
            .await
            .unwrap();
        bot.download_file(&file.path, &mut dst).await.unwrap();
        webm::convert_webm_to_mp4(&webm_filename, &mp4_filename).await;
        bot.send_video(
            msg.chat.id,
            teloxide::types::InputFile::file(std::path::Path::new(&mp4_filename)),
        )
        .reply_to_message_id(msg.id)
        .await?;
        webm::delete_mp4(&mp4_filename).await;
        webm::delete_webm(&webm_filename).await;
    }
    Ok(())
}

/// Handles messages from the bot.
///
/// # Errors
///
/// This function will return an error if .

pub async fn handle_messages(
    bot: &Bot,
    msg: &Message,
    redis_connection: &redis::Client,
) -> Result<(), Box<dyn Error>> {
    match Some(msg) {
        Some(msg) if msg.text().is_some() => {
            process_text_messages(bot, msg, redis_connection, msg.text().unwrap()).await?
        }
        Some(msg) if msg.document().is_some() => {
            process_files(bot, msg, redis_connection, msg.document().unwrap()).await?
        }
        Some(_) => (),
        None => (),
    };
    Ok(())
}

/// Parse messages from the bot.
pub async fn parse_messages(bot: Bot, listener: impl UpdateListener<Err = Infallible> + Send) {
    let redis_client = redis_connection::redis_connection().await.unwrap();
    teloxide::repl_with_listener(
        bot,
        move |bot, msg| {
            let redis_client = redis_client.clone();
            async move {
                if let Err(err) = handle_messages(&bot, &msg, &redis_client).await {
                    error!("Error processing text messages: {}", err);
                }
                Ok(())
            }
        },
        listener,
    )
    .await;
}
