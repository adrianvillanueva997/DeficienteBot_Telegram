#![warn(clippy::pedantic)]

use std::convert::Infallible;

use std::thread;
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

#[instrument]
async fn process_webm_urls(bot: Bot, msg: Message, url: String, redis_connection: redis::Client) {
    thread::spawn(move || {
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            if webm::check_url_status_code(&url).await == Some(200) {
                let uuid = Uuid::new_v4();
                let webm_filename = format!("{uuid}.webm");
                let mp4_filename = format!("{uuid}.mp4");
                bot.send_chat_action(msg.chat.id, teloxide::types::ChatAction::UploadVideo)
                    .await
                    .unwrap();
                webm::download_webm(&url, &webm_filename).await;
                webm::convert_webm_to_mp4(&webm_filename, &mp4_filename).await;
                bot.send_video(
                    msg.chat.id,
                    teloxide::types::InputFile::file(std::path::Path::new(&mp4_filename)),
                )
                .reply_to_message_id(msg.id)
                .await
                .unwrap();
                Rank::new(redis_connection.clone())
                    .update_rank("webm")
                    .await;
            } else {
                bot.send_message(msg.chat.id, "El video no existe 😭")
                    .reply_to_message_id(msg.id)
                    .await
                    .unwrap();
            }
        });
    });
}

fn format_message_username(msg: &Message, content: &str) -> String {
    let message = msg.clone();
    let user = message.from().as_ref().unwrap().username.as_ref().unwrap();
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
    redis_connection: &redis::Client,
    text: &str,
) -> Result<(), Box<dyn Error>> {
    // let message = text.to_lowercase();
    let message = text.to_string();
    let mut actions: Vec<_> = Vec::new();
    if message_checks::url::is_url(&message) {
        let twitter = message_checks::twitter::update_vxtwitter(&message).await;
        if let Some(twitter) = twitter {
            let tweet = format_message_username(msg, &twitter);
            bot.delete_message(msg.chat.id, msg.id).await?;
            Rank::new(redis_connection.clone())
                .update_rank("twitter")
                .await;
            actions.push(bot.send_message(msg.chat.id, tweet));
        } else if webm::is_webm_url(&message) {
            process_webm_urls(
                bot.clone(),
                msg.clone(),
                message.clone(),
                redis_connection.clone(),
            )
            .await;
        } else if message_checks::tiktok::check_if_tiktok(&message) {
            let tntok = message_checks::tiktok::updated_tiktok(&message).await;
            if let Some(tntok) = tntok {
                let tiktok = format_message_username(msg, &tntok);
                Rank::new(redis_connection.clone())
                    .update_rank("tiktok")
                    .await;
                bot.delete_message(msg.chat.id, msg.id).await?;
                actions.push(bot.send_message(msg.chat.id, tiktok));
            }
        }
    }
    let message = message.to_lowercase();
    if bad_words::find_bad_words(&message).await {
        Rank::new(redis_connection.clone())
            .update_rank("uwus")
            .await;
        actions.push(
            bot.send_message(msg.chat.id, "Deficiente")
                .reply_to_message_id(msg.id),
        );
    }
    let (matching_words, copypastas) = message_checks::copypasta::find_copypasta(&message).await;
    for word in matching_words {
        Rank::new(redis_connection.clone()).update_rank(&word).await;
    }

    for copypasta in copypastas {
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
        Rank::new(redis_connection.clone())
            .update_rank("Asuka")
            .await;
        actions.push(
            bot.send_message(msg.chat.id, thursday::random_message().await)
                .reply_to_message_id(msg.id),
        );
    }
    if &message == "deficienteranking" {
        match Rank::new(redis_connection.clone()).get_ranking().await {
            Ok(rank) => {
                let mut ranking_message = String::new();
                for (key, value) in rank {
                    ranking_message.push_str(&format!("{} {}: {}\n", "🔥", key, value));
                }
                actions.push(
                    bot.send_message(msg.chat.id, ranking_message)
                        .reply_to_message_id(msg.id),
                );
            }
            Err(e) => {
                eprintln!("Error getting ranking: {e}");
                // Handle the error appropriately here, e.g., by sending an error message
                actions.push(
                    bot.send_message(
                        msg.chat.id,
                        "Error getting ranking. Please try again later.",
                    )
                    .reply_to_message_id(msg.id),
                );
            }
        }
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
    file_to_read: &Document,
) -> Result<(), Box<dyn Error>> {
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
/// This function will return an error if the bot fails to handle the messages.
///
/// # Panics
///
/// Panics if the bot fails to handle the messages.
pub async fn handle_messages(
    bot: &Bot,
    msg: &Message,
    redis_connection: &redis::Client,
) -> Result<(), Box<dyn Error>> {
    match Some(msg) {
        Some(msg) if msg.text().is_some() => {
            process_text_messages(bot, msg, redis_connection, msg.text().unwrap()).await?;
        }
        Some(msg) if msg.document().is_some() => {
            process_files(bot, msg, redis_connection, msg.document().unwrap()).await?;
        }
        Some(_) | None => (),
    };
    Ok(())
}

/// Parse messages from the bot.
///
/// # Panics
///
/// Panics if the bot fails to parse the messages.
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
