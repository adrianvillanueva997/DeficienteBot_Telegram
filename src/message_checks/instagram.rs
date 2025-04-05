use tracing::instrument;

/// Checks if the url contains an instagram link and converts it to ddinstagram format
/// for better accessibility without the Instagram app.
#[instrument]
pub async fn update_ddinstagram(url: &str) -> Option<String> {
    let url = url.trim();
    if url.contains("instagram") {
        let new_url = url.replace("instagram", "ddinstagram");
        if url.is_empty() {
            return None;
        }
        if let Ok(mut parsed_url) = url::Url::parse(url) {
            if let Some(host) = parsed_url.host_str() {
                if host.contains("instagram") {
                    let new_host = host.replace("instagram", "ddinstagram");
                    if parsed_url.set_host(Some(&new_host)).is_ok() {
                        return Some(parsed_url.to_string());
                    }
                }
            }
            return None;
        }
        Some(new_url)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_instagram_url_conversion() {
        let test_url = "https://www.instagram.com/p/123456";
        assert_eq!(
            update_ddinstagram(test_url).await,
            Some("https://www.ddinstagram.com/p/123456".to_string())
        );
    }

    #[tokio::test]
    async fn test_non_instagram_url() {
        let test_url = "https://www.twitter.com/post";
        assert_eq!(update_ddinstagram(test_url).await, None);
    }

    #[tokio::test]
    async fn test_empty_url() {
        let test_url = "";
        assert_eq!(update_ddinstagram(test_url).await, None);
    }

    #[tokio::test]
    async fn test_instagram_subdomain() {
        let test_url = "https://about.instagram.com";
        assert_eq!(
            update_ddinstagram(test_url).await,
            Some("https://about.ddinstagram.com".to_string())
        );
    }
}
