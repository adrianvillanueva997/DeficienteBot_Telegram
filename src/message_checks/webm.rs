use std::process::Command;

use tokio::{fs::File, io::AsyncWriteExt};
use tokio_stream::StreamExt;
const WEBM: &str = "file.webm";
pub const MP4: &str = "file.mp4";

pub async fn check_url_status_code(url: &str) -> Option<u16> {
    let request = reqwest::get(url).await;
    let response = match request {
        Ok(response) => response,
        Err(_) => return None,
    };
    let status_code = response.status().as_u16();
    Some(status_code)
}

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

pub fn url_is_webm(url: &str) -> bool {
    url.ends_with(".webm")
}

pub async fn convert_webm_to_mp4() {
    Command::new("ffmpeg")
        .arg("-i")
        .arg(WEBM)
        .arg(MP4)
        .output()
        .expect("Failed to execute FFmpeg command");
}

pub async fn delete_webm() {
    std::fs::remove_file(WEBM).expect("Failed to delete webm");
}

pub async fn delete_mp4() {
    std::fs::remove_file(MP4).expect("Failed to delete mp4");
}

pub async fn webm_exists() -> bool {
    std::path::Path::new(WEBM).exists()
}

pub async fn mp4_exists() -> bool {
    std::path::Path::new(MP4).exists()
}
