#![warn(clippy::pedantic)]

use std::convert::Infallible;

use crate::error::BotError;
use std::time::Duration;

use message_checks::friday::fetch_friday_video;
use message_checks::thursday::ThursdayChecker;
use message_checks::tiktok::check_if_tiktok;

use message_checks::{bad_words, webm};
use online_downloads::url_checker::{check_url_status_code, is_mp4_url, is_webm_url};
use online_downloads::video_downloader::{delete_file, download_video};
use prank::day_check::is_prank_day;
use prank::randomizer::should_trigger;
use prank::reverse_words::upside_down_string;
use spotify::client::Spotify;
use spotify_handler::SpotifyHandler;

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
use uuid::Uuid;

mod error;
pub mod message_checks;
pub mod online_downloads;
pub mod prank;
pub mod spotify;
mod spotify_handler;
mod utils;

pub const PRANK_THRESHOLD: u32 = 30;

fn get_telegram_username(msg: &Message) -> String {
    let user = msg.from.as_ref().unwrap().username.as_ref().unwrap();
    format!("@{user}")
}

#[instrument]
async fn process_webm_urls(bot: &Bot, msg: &Message, url: &str) {
    match check_url_status_code(url).await {
        Some(status) if (200..=299).contains(&status) => {
            let uuid = Uuid::new_v4();
            let webm_filename = format!("{uuid}.webm");
            let mp4_filename = format!("{uuid}.mp4");
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
            let uuid = Uuid::new_v4();
            let mp4_filename = format!("{uuid}.mp4");
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

fn format_message_username(msg: &Message, content: &str) -> String {
    let user = msg.from.as_ref().unwrap().username.as_ref().unwrap();
    format!("@{user} \n{content} ")
}

/// Bot logic goes here.
///
/// # Panics
///
/// Panics if the bot fails to run.
///
/// # Errors
///
/// This function will return an error if the bot fails to run.
#[allow(clippy::too_many_lines)]
async fn process_text_messages(
    bot: &Bot,
    msg: &Message,
    text: &str,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut actions: Vec<_> = Vec::new();
    if message_checks::url::is_url(text) {
        let twitter = message_checks::twitter::update_vxtwitter(text).await;
        if let Some(twitter) = twitter {
            let tweet = format_message_username(msg, &twitter);
            bot.delete_message(msg.chat.id, msg.id).await?;
            actions.push(bot.send_message(msg.chat.id, tweet));
        } else if is_webm_url(text) {
            process_webm_urls(bot, msg, text).await;
        } else if check_if_tiktok(text) {
            let tntok = message_checks::tiktok::updated_tiktok(text).await;
            if let Some(tntok) = tntok {
                let tiktok = format_message_username(msg, &tntok);
                bot.delete_message(msg.chat.id, msg.id).await?;
                actions.push(bot.send_message(msg.chat.id, tiktok));
            }
        } else if is_mp4_url(text) {
            process_mp4_urls(bot, msg, text).await;
        } else if let Some(instagram) = message_checks::instagram::update_ddinstagram(text).await {
            let instagram_message = format_message_username(msg, &instagram);
            bot.delete_message(msg.chat.id, msg.id).await?;
            actions.push(bot.send_message(msg.chat.id, instagram_message));
        }
        let spotify_client = Spotify::new().await?;
        let spotify_handler = SpotifyHandler::new(&spotify_client, bot);
        spotify_handler.process_spotify_url(msg, text).await?;
    }
    if is_prank_day() && should_trigger(PRANK_THRESHOLD) {
        if should_trigger(45) {
            let reversed_message = upside_down_string(text);
            bot.delete_message(msg.chat.id, msg.id).await?;
            actions.push(bot.send_message(msg.chat.id, reversed_message));
        } else {
            match prank::mario::fetch_random_image() {
                Ok((caption, image)) => {
                    bot.send_chat_action(msg.chat.id, teloxide::types::ChatAction::UploadPhoto)
                        .await?;
                    // Send photo immediately since we can't queue it
                    bot.send_photo(msg.chat.id, image)
                        .reply_parameters(ReplyParameters::new(msg.id))
                        .caption(caption)
                        .await?;
                }
                Err(e) => {
                    tracing::error!("Failed to fetch random image: {}", e);
                }
            }
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
    let thursday = ThursdayChecker::new();
    if let Some(happy_thursday) = thursday.asuka(&message) {
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
    file_to_read: &Document,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    // Telegram max file size: 20 MB
    if file_to_read.clone().file_name.unwrap().contains("webm")
        && file_to_read.clone().file.size <= 20_000_000
    {
        let uuid = Uuid::new_v4();
        let webm_filename = format!("{uuid}.webm,");
        let mp4_filename = format!("{uuid}.mp4");
        bot.send_chat_action(msg.chat.id, teloxide::types::ChatAction::UploadVideo)
            .await
            .unwrap();
        let file = bot.get_file(file_to_read.file.clone().id).await.unwrap();
        let mut dst = fs::File::create(format!("{webm_filename}.webm"))
            .await
            .unwrap();
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

/// Handles messages from the bot.
///
/// # Errors
///
/// This function will return an error if the bot fails to handle the messages.
///
/// # Panics
///
/// Panics if the bot fails to handle the messages.
pub async fn handle_messages(bot: &Bot, msg: &Message) -> Result<(), BotError> {
    match Some(msg) {
        Some(msg) if msg.text().is_some() => {
            process_text_messages(bot, msg, msg.text().unwrap())
                .await
                .map_err(|e| BotError::Processing(e.to_string()))?;
        }
        Some(msg) if msg.document().is_some() => {
            process_files(bot, msg, msg.document().unwrap())
                .await
                .map_err(|e| BotError::Processing(e.to_string()))?;
        }
        Some(_) | None => (),
    }
    Ok(())
}

/// Parse messages from the bot.
///
/// # Panics
///
/// Panics if the bot fails to parse the messages.
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
