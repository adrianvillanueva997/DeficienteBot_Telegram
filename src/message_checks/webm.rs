use std::process::Command;

use tokio::{fs::File, io::AsyncWriteExt};
use tokio_stream::StreamExt;
pub const WEBM: &str = "file.webm";
pub const MP4: &str = "file.mp4";

/// Checks if the message contains a valid url.
pub async fn check_url_status_code(url: &str) -> Option<u16> {
    let request = reqwest::get(url).await;
    let response = match request {
        Ok(response) => response,
        Err(_) => return None,
    };
    let status_code = response.status().as_u16();
    Some(status_code)
}

/// Downloads a webm from a url.
///
/// # Panics
///
/// Panics if the file can't be created or if the file can't be written to.
pub async fn download_webm(url: &str) {
    let request = reqwest::get(url);
    let response = match request.await {
        Ok(response) => response,
        Err(_) => return,
    };
    let mut file = File::create(WEBM).await.expect("Failed to create file");
    let mut stream = response.bytes_stream();
    while let Some(item) = stream.next().await {
        let item = item.expect("Failed to get item");
        file.write_all(&item)
            .await
            .expect("Failed to write to file");
    }
}

/// Checks if the url ends with .webm.
pub fn url_is_webm(url: &str) -> bool {
    url.ends_with(".webm")
}

/// Converts a webm to mp4.
///
/// # Panics
///
/// Panics if the command fails to execute.
pub async fn convert_webm_to_mp4() {
    Command::new("ffmpeg")
        .arg("-i")
        .arg(WEBM)
        .arg(MP4)
        .output()
        .expect("Failed to execute FFmpeg command");
}

/// Deletes the webm file.
///
/// # Panics
///
/// Panics if the file can't be deleted.
pub async fn delete_webm() {
    std::fs::remove_file(WEBM).expect("Failed to delete webm");
}

/// Deletes the mp4 file.
///
/// # Panics
///
/// Panics if the file can't be deleted.
pub async fn delete_mp4() {
    std::fs::remove_file(MP4).expect("Failed to delete mp4");
}

/// Checks if the webm file exists.
pub async fn webm_exists() -> bool {
    std::path::Path::new(WEBM).exists()
}

/// Checks if the mp4 file exists.
pub async fn mp4_exists() -> bool {
    std::path::Path::new(MP4).exists()
}

pub async fn files_exist() {
    if webm_exists().await {
        delete_webm().await;
    }
    if mp4_exists().await {
        delete_mp4().await;
    }
}
