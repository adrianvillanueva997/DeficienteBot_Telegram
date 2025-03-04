#![warn(clippy::pedantic)]

use std::convert::Infallible;

use std::time::Duration;

use message_checks::friday::fetch_friday_video;
use message_checks::tiktok::check_if_tiktok;

use message_checks::{bad_words, thursday, webm};
use online_downloads::url_checker::{check_url_status_code, is_mp4_url, is_webm_url};
use online_downloads::video_downloader::{delete_file, download_video};
use prank::day_check::is_prank_day;
use prank::randomizer::should_trigger;
use prank::reverse_words::upside_down_string;
use rspotify::model::Market;
use rspotify::prelude::BaseClient;
use spotify::client::{Spotify, SpotifyKind};
use std::error::Error;
use teloxide::net::Download;
use teloxide::payloads::{SendMessageSetters, SendPhotoSetters, SendVideoSetters};
use teloxide::requests::Requester;
use teloxide::types::{Document, InputFile, Message, ReplyParameters};
use teloxide::update_listeners::UpdateListener;
use teloxide::Bot;
use tokio::fs;
use tokio::time::sleep;
use tracing::{error, instrument};
use uuid::Uuid;

pub mod message_checks;
pub mod online_downloads;
pub mod prank;
pub mod spotify;

pub const PRANK_THRESHOLD: u32 = 30;

fn get_telegram_username(msg: &Message) -> String {
    let user = msg.from.as_ref().unwrap().username.as_ref().unwrap();
    format!("@{user}")
}

#[instrument]
async fn process_webm_urls(bot: Bot, msg: Message, url: String) {
    if check_url_status_code(&url).await == Some(200) {
        let uuid = Uuid::new_v4();
        let webm_filename = format!("{uuid}.webm");
        let mp4_filename = format!("{uuid}.mp4");
        bot.send_chat_action(msg.chat.id, teloxide::types::ChatAction::UploadVideo)
            .await
            .unwrap();
        download_video(&url, &webm_filename).await;
        webm::convert_webm_to_mp4(&webm_filename, &mp4_filename).await;
        bot.send_video(
            msg.chat.id,
            teloxide::types::InputFile::file(std::path::Path::new(&mp4_filename)),
        )
        .reply_parameters(ReplyParameters::new(msg.id))
        .await
        .unwrap();
    } else {
        bot.send_message(msg.chat.id, "El video no existe 😭")
            .reply_parameters(ReplyParameters::new(msg.id))
            .await
            .unwrap();
    }
}

#[instrument]
async fn process_mp4_urls(bot: Bot, msg: Message, url: String) {
    if check_url_status_code(&url).await == Some(200) {
        let uuid = Uuid::new_v4();
        let mp4_filename = format!("{uuid}.mp4");
        bot.send_chat_action(msg.chat.id, teloxide::types::ChatAction::UploadVideo)
            .await
            .unwrap();
        download_video(&url, &mp4_filename).await;
        bot.send_video(
            msg.chat.id,
            teloxide::types::InputFile::file(std::path::Path::new(&mp4_filename)),
        )
        .reply_parameters(ReplyParameters::new(msg.id))
        .await
        .unwrap();
        delete_file(&mp4_filename).await;
    } else {
        bot.send_message(msg.chat.id, "El video no existe 😭")
            .reply_parameters(ReplyParameters::new(msg.id))
            .await
            .unwrap();
    }
}

fn format_message_username(msg: &Message, content: &str) -> String {
    let user = msg.from.as_ref().unwrap().username.as_ref().unwrap();
    format!("@{user} \n{content} ")
}

#[instrument]
async fn prepare_album_content(
    spotify_client: Spotify,
    bot: &Bot,
    msg: &Message,
    album_id: String,
    spotify_url: &String
) -> Result<(), Box<dyn Error>> {
    let album = spotify_client
        .client
        .album(
            rspotify::model::AlbumId::from_id(&album_id)
                .map_err(|e| format!("Invalid album ID: {e}"))?,
            Some(Market::Country(rspotify::model::Country::Spain)),
        )
        .await?;

    // Format artists
    let artists = album
        .artists
        .iter()
        .map(|artist| artist.name.as_str())
        .collect::<Vec<_>>()
        .join(", ");

    // Format genres
    let genres = if album.genres.is_empty() {
        "N/A".to_string()
    } else {
        album.genres.join(", ")
    };

    // Calculate total duration
    let total_duration: u64 = album
        .tracks
        .items
        .iter()
        .map(|track| track.duration.num_milliseconds().unsigned_abs())
        .sum();
    let duration = Duration::from_millis(total_duration);
    let hours = duration.as_secs() / 3600;
    let minutes = (duration.as_secs() % 3600) / 60;

    // Format tracks with duration and explicit marks
    let tracks = album
        .tracks
        .items
        .iter()
        .map(|track| {
            let duration = Duration::from_millis(track.duration.num_milliseconds().unsigned_abs());
            let minutes = duration.as_secs() / 60;
            let seconds = duration.as_secs() % 60;
            let explicit_mark = if track.explicit { "🔞 " } else { "" };
            format!(
                "{}. {} {} ({:02}:{:02})",
                track.track_number, explicit_mark, track.name, minutes, seconds
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    // Get album cover
    let cover_url = album
        .images
        .first()
        .ok_or("No album image available")?
        .url
        .as_str();

    let content = format!(
        "*Sent by:* {}\n\
        🎨 *Album:* {}\n\
        🎼 *Album Type:* {}\n\
        👥 *Artists:* {}\n\
        📅 *Release Date:* {}\n\
        🎵 *Total Tracks:* {}\n\
        ⏱️ *Duration:* {}h {}m\n\
        🏷️ *Label:* {}\n\
        🎭 *Genres:* {}\n\
        ⭐ *Popularity:* {}/100\n\n\
        *Tracks:*\n\
        {}\n\n\
        [🔗 Open in Spotify]({})",
        escape_markdown(&get_telegram_username(msg)),
        escape_markdown(&album.name),
        escape_markdown((&album.album_type).into()),
        escape_markdown(&artists),
        escape_markdown(&album.release_date),
        escape_markdown(&album.tracks.total.to_string()),
        escape_markdown(&hours.to_string()),
        escape_markdown(&minutes.to_string()),
        escape_markdown(album.label.as_deref().unwrap_or("N/A")),
        escape_markdown(&genres),
        escape_markdown(&album.popularity.to_string()),
        escape_markdown(&tracks),
        escape_markdown(spotify_url)
    );

    bot.send_chat_action(msg.chat.id, teloxide::types::ChatAction::UploadPhoto)
        .await
        .unwrap();
    // Send photo and delete original message only if successful
    match bot
        .send_photo(
            msg.chat.id,
            InputFile::url(url::Url::parse(cover_url).unwrap()),
        )
        .reply_parameters(ReplyParameters::new(msg.id))
        .caption(content)
        .parse_mode(teloxide::types::ParseMode::MarkdownV2)
        .await
    {
        Ok(_) => {
            // Delete original message only after successful send
            bot.delete_message(msg.chat.id, msg.id).await?;
            Ok(())
        }
        Err(e) => {
            error!("Failed to send Spotify album content: {}", e);
            Err(Box::new(e))
        }
    }
}

fn escape_markdown(text: &str) -> String {
    let special_chars = [
        '_', '*', '[', ']', '(', ')', '~', '`', '>', '#', '+', '-', '=', '|', '{', '}', '.', '!',
    ];
    let mut result = String::with_capacity(text.len() * 2);
    for c in text.chars() {
        if special_chars.contains(&c) {
            result.push('\\');
        }
        result.push(c);
    }
    result
}

#[instrument]
async fn prepare_artist_content(
    spotify_client: Spotify,
    bot: &Bot,
    msg: &Message,
    artist_id: String,
    spotify_url: &String
) -> Result<(), Box<dyn Error>> {
    let artist_data = spotify_client
        .client
        .artist(
            rspotify::model::ArtistId::from_id(&artist_id)
                .map_err(|e| format!("Invalid artist ID: {e}"))?,
        )
        .await?;

    // Get artist image
    let artist_image = artist_data
        .images
        .first()
        .ok_or("No artist image available")?
        .url
        .as_str();

    // Format genres list
    let genres = if artist_data.genres.is_empty() {
        "N/A".to_string()
    } else {
        artist_data.genres.join(", ")
    };

    // Format follower count with thousand separators
    let followers = artist_data
        .followers
        .total
        .to_string()
        .chars()
        .rev()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("'")
        .chars()
        .rev()
        .collect::<String>();

    let content = format!(
        "*Sent by:* {}\n\
👤 *Artist:* {}\n\
🎭 *Genres:* {}\n\
👥 *Followers:* {}\n\
⭐ *Popularity:* {}/100\n\
[🔗 Open in Spotify]({})",
        escape_markdown(&get_telegram_username(msg)),
        escape_markdown(&artist_data.name),
        escape_markdown(&genres),
        escape_markdown(&followers),
        escape_markdown(&artist_data.popularity.to_string()),
        escape_markdown(spotify_url)
    );

    bot.send_chat_action(msg.chat.id, teloxide::types::ChatAction::UploadPhoto)
        .await?;

    bot.send_photo(msg.chat.id, InputFile::url(url::Url::parse(artist_image)?))
        .reply_parameters(ReplyParameters::new(msg.id))
        .caption(content)
        .parse_mode(teloxide::types::ParseMode::MarkdownV2)
        .await?;

    Ok(())
}

#[instrument]
async fn prepare_playlist_content(
    spotify_client: Spotify,
    bot: &Bot,
    msg: &Message,
    playlist_id: String,
    spotify_url: &String,
) -> Result<(), Box<dyn Error>> {
    let playlist_data = spotify_client
        .client
        .playlist(
            rspotify::model::PlaylistId::from_id(&playlist_id)
                .map_err(|e| format!("Invalid playlist ID: {e}"))?,
            None,
            Some(Market::Country(rspotify::model::Country::Spain)),
        )
        .await?;

    let playlist_image = playlist_data
        .images
        .first()
        .ok_or("No playlist image available")?
        .url
        .as_str();

    let collaborative_mark = if playlist_data.collaborative {
        "👥 "
    } else {
        ""
    };
    let public_mark = if playlist_data.public.unwrap_or(true) {
        "🌐 "
    } else {
        "🔒 "
    };

    let content = format!(
        "*Sent by:* {}\n\
📜 *Playlist:* {}{}{}\n\
👤 *Created by:* {}\n\
📝 *Description:* {}\n\
🎵 *Total Tracks:* {}\n\
👥 *Followers:* {}\n\
📅 *Last Modified:* {}\n\
[🔗 Open in Spotify]({})",
        escape_markdown(&get_telegram_username(msg)),
        collaborative_mark,
        public_mark,
        escape_markdown(&playlist_data.name),
        escape_markdown(&playlist_data.owner.display_name.unwrap_or_default()),
        escape_markdown(&playlist_data.description.unwrap_or_default()),
        escape_markdown(&playlist_data.tracks.total.to_string()),
        escape_markdown(&playlist_data.followers.total.to_string()),
        escape_markdown(&playlist_data.snapshot_id),
        escape_markdown(spotify_url)
    );

    bot.send_chat_action(msg.chat.id, teloxide::types::ChatAction::UploadPhoto)
        .await?;

    bot.send_photo(
        msg.chat.id,
        InputFile::url(url::Url::parse(playlist_image)?),
    )
    .reply_parameters(ReplyParameters::new(msg.id))
    .caption(content)
    .parse_mode(teloxide::types::ParseMode::MarkdownV2)
    .await?;

    Ok(())
}
#[instrument]
async fn prepare_track_content(
    spotify_client: Spotify,
    bot: &Bot,
    msg: &Message,
    track_id: String,
    spotify_url: &String,
) -> Result<(), Box<dyn Error>> {
    let track_data = spotify_client
        .client
        .track(
            rspotify::model::TrackId::from_id(&track_id)
                .map_err(|e| format!("Invalid track ID: {e}"))?,
            Some(Market::Country(rspotify::model::Country::Spain)),
        )
        .await;

    match track_data {
        Ok(track) => {
            // Format artists
            let artists = track
                .artists
                .iter()
                .map(|artist| artist.name.clone())
                .collect::<Vec<String>>()
                .join(", ");

            // Get album image (if available)
            let track_image = track
                .album
                .images
                .first()
                .ok_or("No album image available")?
                .url
                .as_str();

            // Optional: Show preview URL if available
            let preview_section = track.preview_url.as_ref().map_or(String::new(), |url| {
                format!("\n🔊 [Preview]({})", escape_markdown(url))
            });

            let duration = Duration::from_millis(track.duration.num_milliseconds().unsigned_abs());
            let minutes = duration.as_secs() / 60;
            let seconds = duration.as_secs() % 60;
            let explicit_mark = if track.explicit { "🔞 " } else { "" };

            let content = format!(
                "*Sent by:* {}\n\
🎵 *Track:* {}{}\n\
🎤 *Artists:* {}\n\
💽 *Album:* {}\n\
⏱️ *Duration:* {:02}:{:02}\n\
📅 *Release Date:* {}\n\
🔢 *Track Number:* {}\n\
⭐ *Popularity:* {}/100\n\
[🎧 Open in Spotify]({}){}",
                escape_markdown(&get_telegram_username(msg)),
                explicit_mark,
                escape_markdown(&track.name),
                escape_markdown(&artists),
                escape_markdown(&track.album.name),
                minutes,
                seconds,
                escape_markdown(&track.album.release_date.unwrap_or_default()),
                escape_markdown(&track.track_number.to_string()),
                escape_markdown(&track.popularity.to_string()),
                escape_markdown(spotify_url),
                preview_section
            );

            bot.send_chat_action(msg.chat.id, teloxide::types::ChatAction::UploadPhoto)
                .await?;

            bot.send_photo(msg.chat.id, InputFile::url(url::Url::parse(track_image)?))
                .reply_parameters(ReplyParameters::new(msg.id))
                .caption(content)
                .parse_mode(teloxide::types::ParseMode::MarkdownV2)
                .await?;

            Ok(())
        }
        Err(e) => {
            error!("Failed to fetch track data: {}", e);
            bot.send_message(msg.chat.id, "Failed to fetch track data")
                .reply_parameters(ReplyParameters::new(msg.id))
                .await?;
            Err(e.into())
        }
    }
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
async fn process_text_messages(bot: &Bot, msg: &Message, text: &str) -> Result<(), Box<dyn Error>> {
    let message = text.to_string();
    let mut actions: Vec<_> = Vec::new();
    if message_checks::url::is_url(&message) {
        let twitter = message_checks::twitter::update_vxtwitter(&message).await;
        if let Some(twitter) = twitter {
            let tweet = format_message_username(msg, &twitter);
            bot.delete_message(msg.chat.id, msg.id).await?;
            actions.push(bot.send_message(msg.chat.id, tweet));
        } else if is_webm_url(&message) {
            process_webm_urls(bot.clone(), msg.clone(), message.clone()).await;
        } else if check_if_tiktok(&message) {
            let tntok = message_checks::tiktok::updated_tiktok(&message).await;
            if let Some(tntok) = tntok {
                let tiktok = format_message_username(msg, &tntok);
                bot.delete_message(msg.chat.id, msg.id).await?;
                actions.push(bot.send_message(msg.chat.id, tiktok));
            }
        } else if is_mp4_url(&message) {
            process_mp4_urls(bot.clone(), msg.clone(), message.clone()).await;
        }
        let spotify = Spotify::new().await?;
        let spotify_url = spotify.identify_spotify_url(&message);
        if spotify_url != SpotifyKind::Unknown {
            if let Some(id) = spotify.extract_spotify_id(&message) {
                match spotify_url {
                    SpotifyKind::Album => {
                        prepare_album_content(spotify, bot, msg, id, &message).await?;
                    }
                    SpotifyKind::Artist => prepare_artist_content(spotify, bot, msg, id,&message).await?,
                    SpotifyKind::Playlist => {
                        prepare_playlist_content(spotify, bot, msg, id,&message).await?;
                    }
                    SpotifyKind::Track => prepare_track_content(spotify, bot, msg, id,&message).await?,
                    SpotifyKind::Unknown => todo!(),
                }
            }
        }
    }
    if is_prank_day() && should_trigger(PRANK_THRESHOLD) {
        if should_trigger(45) {
            let reversed_message = upside_down_string(&message);
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
    let message = message.to_lowercase();
    if bad_words::find_bad_words(&message).await {
        actions.push(
            bot.send_message(msg.chat.id, "Deficiente")
                .reply_parameters(ReplyParameters::new(msg.id)),
        );
    }
    let (_matching_words, copypastas) = message_checks::copypasta::find_copypasta(&message).await;

    for copypasta in copypastas {
        if copypasta == "viernes" {
            bot.send_chat_action(msg.chat.id, teloxide::types::ChatAction::UploadVideo)
                .await?;
            bot.send_video(msg.chat.id, fetch_friday_video().unwrap())
                .reply_parameters(ReplyParameters::new(msg.id))
                .await?;
            // bot.send_audio(
            //     msg.chat.id,
            //     // TODO: Move this to embedded in the binary
            //     teloxide::types::InputFile::file(std::path::Path::new("viernes.ogg")),
            // )
            // .reply_parameters(ReplyParameters::new(msg.id))
            // .await?;
        } else {
            actions.push(
                bot.send_message(msg.chat.id, copypasta)
                    .reply_parameters(ReplyParameters::new(msg.id)),
            );
        }
    }

    if thursday::is_thursday().await && thursday::check_asuka(&message).await {
        actions.push(
            bot.send_message(msg.chat.id, thursday::random_message().await)
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
#[allow(clippy::match_same_arms)]
pub async fn handle_messages(bot: &Bot, msg: &Message) -> Result<(), Box<dyn Error>> {
    match Some(msg) {
        Some(msg) if msg.text().is_some() => {
            process_text_messages(bot, msg, msg.text().unwrap()).await?;
        }
        Some(msg) if msg.document().is_some() => {
            process_files(bot, msg, msg.document().unwrap()).await?;
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
