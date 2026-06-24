#![warn(clippy::pedantic)]

use std::convert::Infallible;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::error::BotError;
use message_checks::friday::fetch_friday_video;
use message_checks::{bad_words, webm};
use online_downloads::url_checker::{check_url_status_code, is_mp4_url, is_webm_url};
use online_downloads::video_downloader::{delete_file, download_video};
use prank::day_check::is_prank_day;
use prank::randomizer::should_trigger;
use prank::reverse_words::upside_down_string;
use prank::sarcastic_agree::random_sarcastic_reply;
use std::error::Error;
use teloxide::net::Download;
use teloxide::payloads::{SendMessageSetters, SendPhotoSetters, SendVideoSetters};
use teloxide::requests::Requester;
use teloxide::types::{Document, Message, ReplyParameters};
use teloxide::update_listeners::UpdateListener;
use teloxide::Bot;
use tokio::fs;
use tokio::time::sleep;
use tracing::{error, instrument};

mod error;
pub mod message_checks;
pub mod online_downloads;
pub mod prank;
mod social_media_handler;
mod utils;

pub const PRANK_THRESHOLD: u32 = 10;

fn unique_filename(ext: &str) -> String {
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    format!("{ts}.{ext}")
}

#[instrument]
async fn process_webm_urls(bot: &Bot, msg: &Message, url: &str) {
    match check_url_status_code(url).await {
        Some(status) if (200..=299).contains(&status) => {
            let webm_filename = unique_filename("webm");
            let mp4_filename = unique_filename("mp4");
            bot.send_chat_action(msg.chat.id, teloxide::types::ChatAction::UploadVideo)
                .await
                .unwrap();
            download_video(url, &webm_filename).await;
            webm::convert_webm_to_mp4(&webm_filename, &mp4_filename).await;
            bot.send_video(
                msg.chat.id,
                teloxide::types::InputFile::file(std::path::Path::new(&mp4_filename)),
            )
            .reply_parameters(ReplyParameters::new(msg.id))
            .await
            .unwrap();
        }
        _ => {
            bot.send_message(msg.chat.id, "El video no existe o no está disponible 😭")
                .reply_parameters(ReplyParameters::new(msg.id))
                .await
                .unwrap();
        }
    }
}

#[instrument]
async fn process_mp4_urls(bot: &Bot, msg: &Message, url: &str) {
    match check_url_status_code(url).await {
        Some(status) if (200..=299).contains(&status) => {
            let mp4_filename = unique_filename("mp4");
            bot.send_chat_action(msg.chat.id, teloxide::types::ChatAction::UploadVideo)
                .await
                .unwrap();
            download_video(url, &mp4_filename).await;
            bot.send_video(
                msg.chat.id,
                teloxide::types::InputFile::file(std::path::Path::new(&mp4_filename)),
            )
            .reply_parameters(ReplyParameters::new(msg.id))
            .await
            .unwrap();
            delete_file(&mp4_filename).await;
        }
        _ => {
            bot.send_message(msg.chat.id, "El video no existe o no está disponible 😭")
                .reply_parameters(ReplyParameters::new(msg.id))
                .await
                .unwrap();
        }
    }
}

#[allow(clippy::too_many_lines)]
async fn process_text_messages(
    bot: &Bot,
    msg: &Message,
    text: &str,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut actions: Vec<_> = Vec::new();
    if message_checks::url::is_url(text) {
        social_media_handler::process(bot, msg, text).await?;
        if is_webm_url(text) {
            process_webm_urls(bot, msg, text).await;
        }
        if is_mp4_url(text) {
            process_mp4_urls(bot, msg, text).await;
        }
    }
    if is_prank_day() && should_trigger(PRANK_THRESHOLD) {
        if should_trigger(15) {
            let reversed_message = upside_down_string(text);
            actions.push(
                bot.send_message(msg.chat.id, reversed_message)
                    .reply_parameters(ReplyParameters::new(msg.id)),
            );
        } else if should_trigger(70) {
            let reply = random_sarcastic_reply();
            actions.push(
                bot.send_message(msg.chat.id, reply)
                    .reply_parameters(ReplyParameters::new(msg.id)),
            );
        } else if let Some((caption, image)) = prank::mario::fetch_random_image() {
            bot.send_chat_action(msg.chat.id, teloxide::types::ChatAction::UploadPhoto)
                .await?;
            bot.send_photo(msg.chat.id, image)
                .reply_parameters(ReplyParameters::new(msg.id))
                .caption(caption)
                .await?;
        }
    }
    let message = text.to_lowercase();
    if bad_words::find_bad_words(&message).await {
        actions.push(
            bot.send_message(msg.chat.id, "Deficiente")
                .reply_parameters(ReplyParameters::new(msg.id)),
        );
    }
    let copypastas = message_checks::copypasta::find_copypasta(&message).await;
    for copypasta in copypastas {
        if copypasta.trigger == "viernes" {
            bot.send_chat_action(msg.chat.id, teloxide::types::ChatAction::UploadVideo)
                .await?;
            bot.send_video(msg.chat.id, fetch_friday_video().unwrap())
                .reply_parameters(ReplyParameters::new(msg.id))
                .await?;
        } else {
            actions.push(
                bot.send_message(msg.chat.id, copypasta.response)
                    .reply_parameters(ReplyParameters::new(msg.id)),
            );
        }
    }
    if let Some(happy_thursday) = message_checks::thursday::check_thursday(&message) {
        actions.push(
            bot.send_message(msg.chat.id, happy_thursday)
                .reply_parameters(ReplyParameters::new(msg.id)),
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

#[allow(clippy::missing_panics_doc, clippy::missing_errors_doc)]
#[instrument]
pub async fn process_files(
    bot: &Bot,
    msg: &Message,
    file_to_read: &Document,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    if file_to_read
        .file_name
        .as_deref()
        .is_some_and(|name| name.contains("webm"))
        && file_to_read.file.size <= 20_000_000
    {
        let webm_filename = unique_filename("webm");
        let mp4_filename = unique_filename("mp4");
        bot.send_chat_action(msg.chat.id, teloxide::types::ChatAction::UploadVideo)
            .await
            .unwrap();
        let file = bot.get_file(file_to_read.file.id.clone()).await.unwrap();
        let mut dst = fs::File::create(&webm_filename).await.unwrap();
        bot.download_file(&file.path, &mut dst).await.unwrap();
        webm::convert_webm_to_mp4(&webm_filename, &mp4_filename).await;
        bot.send_video(
            msg.chat.id,
            teloxide::types::InputFile::file(std::path::Path::new(&mp4_filename)),
        )
        .reply_parameters(ReplyParameters::new(msg.id))
        .await?;
        delete_file(&mp4_filename).await;
        delete_file(&webm_filename).await;
    }
    Ok(())
}

#[allow(clippy::missing_errors_doc)]
#[instrument]
pub async fn handle_messages(bot: &Bot, msg: &Message) -> Result<(), BotError> {
    if let Some(text) = msg.text() {
        process_text_messages(bot, msg, text)
            .await
            .map_err(|e| BotError::Processing(e.to_string()))?;
    } else if let Some(doc) = msg.document() {
        process_files(bot, msg, doc)
            .await
            .map_err(|e| BotError::Processing(e.to_string()))?;
    }
    Ok(())
}

pub async fn parse_messages(bot: Bot, listener: impl UpdateListener<Err = Infallible> + Send) {
    teloxide::repl_with_listener(
        bot,
        move |bot, msg| async move {
            if let Err(err) = handle_messages(&bot, &msg).await {
                error!("Error processing text messages: {}", err);
            }
            Ok(())
        },
        listener,
    )
    .await;
}
