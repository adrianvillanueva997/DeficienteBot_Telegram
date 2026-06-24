use tokio::io::AsyncWriteExt;
use tracing::instrument;

#[allow(clippy::missing_panics_doc)]
#[instrument]
pub async fn download_video(url: &str, output_filename: &str) {
    if let Ok(response) = reqwest::get(url).await {
        let bytes = response.bytes().await.expect("Failed to read response");
        let mut file = tokio::fs::File::create(output_filename)
            .await
            .expect("Failed to create file");
        file.write_all(&bytes)
            .await
            .expect("Failed to write to file");
    }
}

#[allow(clippy::missing_panics_doc)]
#[instrument]
pub async fn delete_file(filename: &str) {
    std::fs::remove_file(filename).expect("Failed to delete mp4");
}
