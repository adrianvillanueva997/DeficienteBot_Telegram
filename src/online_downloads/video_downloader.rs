use tokio::{fs::File, io::AsyncWriteExt};
use tokio_stream::StreamExt;
use tracing::instrument;

#[instrument]
pub async fn download_video(url: &str, output_filename: &str) {
    let request = reqwest::get(url);
    if let Ok(response) = request.await {
        let mut file = File::create(output_filename)
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
}

#[instrument]
pub async fn delete_file(filename: &str) {
    std::fs::remove_file(filename).expect("Failed to delete mp4");
}
