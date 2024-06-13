use std::process::Command;
use tracing::instrument;

use crate::online_downloads::video_downloader::delete_file;

/// Converts a webm to mp4.
///
/// # Panics
///
/// Panics if the command fails to execute.
#[instrument]
pub async fn convert_webm_to_mp4(webm_filename: &str, mp4_filename: &str) {
    Command::new("ffmpeg")
        .arg("-i")
        .arg(webm_filename)
        .arg(mp4_filename)
        .output()
        .expect("Failed to execute FFmpeg command");
}

/// Checks if the webm file exists.
#[instrument]
pub async fn webm_exists(webm_filename: &str) -> bool {
    std::path::Path::new(&webm_filename).exists()
}

/// Checks if the mp4 file exists.
#[instrument]
pub async fn mp4_exists(mp4_filename: &str) -> bool {
    std::path::Path::new(&mp4_filename).exists()
}

#[instrument]
pub async fn files_exist(webm_filename: &str, mp4_filename: &str) {
    if webm_exists(webm_filename).await {
        delete_file(webm_filename).await;
    }
    if mp4_exists(mp4_filename).await {
        delete_file(mp4_filename).await;
    }
}
