use crate::{get_telegram_username, spotify::client::Spotify};
use rspotify::{model::Market, prelude::BaseClient};
use std::{error::Error, time::Duration};
use teloxide::{
    payloads::{SendMessageSetters, SendPhotoSetters},
    prelude::Requester,
    types::{InputFile, Message, ReplyParameters},
    Bot,
};
use tracing::error;
use tracing::instrument;

#[instrument]
pub async fn prepare_album_content(
    spotify_client: Spotify,
    bot: &Bot,
    msg: &Message,
    album_id: String,
    spotify_url: &String,
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
pub async fn prepare_artist_content(
    spotify_client: Spotify,
    bot: &Bot,
    msg: &Message,
    artist_id: String,
    spotify_url: &String,
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
        .caption(content)
        .parse_mode(teloxide::types::ParseMode::MarkdownV2)
        .await?;
    bot.delete_message(msg.chat.id, msg.id).await?;

    Ok(())
}

#[instrument]
pub async fn prepare_playlist_content(
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
    .caption(content)
    .parse_mode(teloxide::types::ParseMode::MarkdownV2)
    .await?;
    bot.delete_message(msg.chat.id, msg.id).await?;

    Ok(())
}
#[instrument]
pub async fn prepare_track_content(
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
                .caption(content)
                .parse_mode(teloxide::types::ParseMode::MarkdownV2)
                .await?;

            bot.delete_message(msg.chat.id, msg.id).await?;

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
