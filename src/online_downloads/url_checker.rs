use tracing::instrument;
use tracing::{error, info};
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

#[must_use]
pub fn is_mp4_url(url: &str) -> bool {
    std::path::Path::new(url)
        .extension()
        .map_or(false, |ext| ext.eq_ignore_ascii_case("mp4"))
}

#[must_use]
pub fn is_webm_url(url: &str) -> bool {
    std::path::Path::new(url)
        .extension()
        .map_or(false, |ext| ext.eq_ignore_ascii_case("webm"))
}
