use regex::Regex;
use std::sync::LazyLock;
use tracing::instrument;

static TWITTER_PATTERNS: LazyLock<Vec<(Regex, &str)>> = LazyLock::new(|| {
    vec![
        // Twitter URLs
        (
            Regex::new(r"https?://(?:www\.)?twitter\.com/(\w+/status/\d+)").unwrap(),
            "https://fxtwitter.com/$1",
        ),
        // X.com URLs
        (
            Regex::new(r"https?://(?:www\.)?x\.com/(\w+/status/\d+)").unwrap(),
            "https://fixupx.com/$1",
        ),
    ]
});

/// Converts Twitter/X URLs to their viewer-friendly alternatives.
///
/// # Arguments
/// * `message` - The message containing potential Twitter/X URLs
///
/// # Returns
/// * `Some(String)` - If a URL was found and converted
/// * `None` - If no convertible URLs were found
/// ```
#[instrument]
pub async fn update_twitter_links(message: &str) -> Option<String> {
    let mut modified = message.to_string();
    let mut was_modified = false;

    for (pattern, replacement) in TWITTER_PATTERNS.iter() {
        if pattern.is_match(&modified) {
            modified = pattern.replace_all(&modified, *replacement).to_string();
            was_modified = true;
        }
    }

    was_modified.then_some(modified)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_twitter_conversion() {
        let tests = vec![
            (
                "https://twitter.com/user/status/123456",
                Some("https://fxtwitter.com/user/status/123456".to_string()),
            ),
            (
                "https://x.com/user/status/123456",
                Some("https://fixupx.com/user/status/123456".to_string()),
            ),
            (
                "Check this tweet: https://twitter.com/user/status/123456",
                Some("Check this tweet: https://fxtwitter.com/user/status/123456".to_string()),
            ),
            ("No twitter link here", None),
            (
                "Already converted: https://fxtwitter.com/user/status/123456",
                None,
            ),
        ];

        for (input, expected) in tests {
            assert_eq!(update_twitter_links(input).await, expected);
        }
    }
}
