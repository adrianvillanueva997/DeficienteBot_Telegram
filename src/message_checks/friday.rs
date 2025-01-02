use rust_embed::RustEmbed;
use teloxide::types::InputFile;



#[derive(RustEmbed)]
#[folder = "assets/videos/"]
struct Asset;

/// Fetches the Friday video from the embedded assets.
///
/// # Errors
///
/// This function will return an error if the video file is not found in the embedded assets.
pub fn fetch_friday_video() -> Result<InputFile, anyhow::Error> {
    let video = Asset::get("viernes.mp4").ok_or_else(|| anyhow::anyhow!("Video not found"))?;
    Ok(InputFile::memory(video.data.into_owned()))
}
