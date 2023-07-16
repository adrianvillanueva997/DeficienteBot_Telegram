use url::Url;

/// Checks if the message contains a valid url.
pub fn is_url(url: &str) -> bool {
    Url::parse(url).is_ok()
}
