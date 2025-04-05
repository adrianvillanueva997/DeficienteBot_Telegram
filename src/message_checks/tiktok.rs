use regex::Regex;
use std::sync::LazyLock;
use tracing::instrument;

static TIKTOK_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"https?://(?:www\.)?(?:vm\.)?tiktok\.com/(@?[^/\s]+/?)+").unwrap()
});

const TIKTOK_REPLACEMENT: &str = "tnktok";

/// Converts TikTok URLs to their alternative viewer-friendly format.
///
/// # Arguments
/// * `message` - Text that may contain TikTok URLs
///
/// # Returns
/// * `Some(String)` - If TikTok URLs were found and converted
/// * `None` - If no TikTok URLs were found
///
/// # Example
/// ```
/// let msg = "Check this: https://tiktok.com/@user/video/123456";
/// assert_eq!(
///     updated_tiktok(msg).unwrap(),
///     "Check this: https://tnktok.com/@user/video/123456"
/// );
/// ```
#[instrument]
pub async fn updated_tiktok(message: &str) -> Option<String> {
    if TIKTOK_PATTERN.is_match(message) {
        Some(message.replace("tiktok", TIKTOK_REPLACEMENT))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_tiktok_conversion() {
        let test_cases = vec![
            (
                "https://tiktok.com/@user/video/123456",
                Some("https://tnktok.com/@user/video/123456".to_string()),
            ),
            (
                "https://vm.tiktok.com/video/123456",
                Some("https://vm.tnktok.com/video/123456".to_string()),
            ),
            (
                "Check this: https://www.tiktok.com/@user/video/123456",
                Some("Check this: https://www.tnktok.com/@user/video/123456".to_string()),
            ),
            ("No tiktok link here", None),
            (
                "Multiple links: https://tiktok.com/1 and https://tiktok.com/2",
                Some("Multiple links: https://tnktok.com/1 and https://tnktok.com/2".to_string()),
            ),
        ];

        for (input, expected) in test_cases {
            assert_eq!(updated_tiktok(input).await, expected);
        }
    }

    #[tokio::test]
    async fn test_invalid_urls() {
        let invalid_urls = vec!["notiktokhere", "http://fake-tiktok.com", "tiktok", ""];

        for url in invalid_urls {
            assert_eq!(updated_tiktok(url).await, None);
        }
    }

    #[tokio::test]
    async fn test_url_variants() {
        let variants = vec![
            "https://tiktok.com",
            "http://tiktok.com",
            "https://www.tiktok.com",
            "https://vm.tiktok.com",
        ];

        for url in variants {
            let result = updated_tiktok(url).await;
            assert!(result.is_some());
            assert!(!result.unwrap().contains("tiktok"));
        }
    }
}
