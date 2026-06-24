use rust_embed::RustEmbed;
use teloxide::types::InputFile;

const FRIDAY_VIDEO_FILENAME: &str = "viernes.mp4";

#[derive(RustEmbed)]
#[folder = "assets/videos/"]
struct Asset;

#[must_use]
pub fn fetch_friday_video() -> Option<InputFile> {
    Asset::get(FRIDAY_VIDEO_FILENAME)
        .map(|video| InputFile::memory(video.data.into_owned()))
}
