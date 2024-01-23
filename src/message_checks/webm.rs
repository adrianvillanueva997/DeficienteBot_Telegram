use std::process::Command;
use tokio::{fs::File, io::AsyncWriteExt};
use tokio_stream::StreamExt;
use tracing::{error, info, instrument};

/// Checks if the message contains a valid url.
#[instrument]
pub async fn check_url_status_code(url: &str) -> Option<u16> {
    let request = reqwest::get(url).await;
    let response = match request {
        Ok(response) => response,
        Err(err) => {
            error!("Failed to get response: {}", err);
            return None;
        }
    };
    let status_code = response.status().as_u16();
    info!("Status code: {}", status_code);
    Some(status_code)
}

/// Downloads a webm from a url.
///
/// # Panics
///
/// Panics if the file can't be created or if the file can't be written to.
#[instrument]
pub async fn download_webm(url: &str, webm_filename: &str) {
    let request = reqwest::get(url);
    let response = match request.await {
        Ok(response) => response,
        Err(_) => return,
    };
    let mut file = File::create(webm_filename)
        .await
        .expect("Failed to create file");
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
#[instrument]
pub async fn convert_webm_to_mp4(webm_filename: &str, mp4_filename: &str) {
    Command::new("ffmpeg")
        .arg("-i")
        .arg(webm_filename)
        .arg(mp4_filename)
        .output()
        .expect("Failed to execute FFmpeg command");
}

/// Deletes the webm file.
///
/// # Panics
///
/// Panics if the file can't be deleted.
#[instrument]
pub async fn delete_webm(webm_filename: &str) {
    std::fs::remove_file(webm_filename).expect("Failed to delete webm");
}

/// Deletes the mp4 file.
///
/// # Panics
///
/// Panics if the file can't be deleted.
#[instrument]
pub async fn delete_mp4(mp4_filename: &str) {
    std::fs::remove_file(mp4_filename).expect("Failed to delete mp4");
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
        delete_webm(webm_filename).await;
    }
    if mp4_exists(mp4_filename).await {
        delete_mp4(mp4_filename).await;
    }
}
