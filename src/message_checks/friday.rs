use rust_embed::RustEmbed;
use teloxide::types::InputFile;

const FRIDAY_VIDEO_FILENAME: &str = "viernes.mp4";


#[derive(RustEmbed)]
#[folder = "assets/videos/"]
struct Asset;

/// Fetches the Friday celebration video from the embedded assets.
///
/// # Examples
///
/// ```
/// use crate::message_checks::friday::fetch_friday_video;
///
/// let video = fetch_friday_video().expect("Failed to load Friday video");
/// ```
///
/// # Errors
///
/// Returns `FridayError::AssetNotFound` if the video file is not found in the embedded assets.
pub fn fetch_friday_video() -> Result<InputFile,anyhow::Error > {
    Asset::get(FRIDAY_VIDEO_FILENAME)
        .map(|video| InputFile::memory(video.data.into_owned()))
        .ok_or_else(|| anyhow::anyhow!("Video not found"))
}
