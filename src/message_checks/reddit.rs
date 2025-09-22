use regex::{Captures, Regex};
use std::sync::LazyLock;
use tracing::instrument;

/// Module for checking and updating Reddit URLs in messages.
///
/// This module provides functionality to detect Reddit URLs and replace them
/// with old.reddit equivalents to avoid modern Reddit's interface.
static REDDIT_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    // Matches: https://reddit.com, https://www.reddit.com, https://m.reddit.com (and http)
    // Captures scheme, optional www/m subdomain, and the rest of the path.
    Regex::new(r"(?:(?P<www>www)\.|(?P<m>m)\.)?reddit\.com(?P<path>/\S*)?").unwrap()
});

const REDDIT_REPLACEMENT: &str = "old.reddit";

/// Updates Reddit URLs in a message to use old.reddit.
///
/// - m.reddit.com -> old.reddit.com (drops the m)
/// - www.reddit.com -> www.old.reddit.com (keeps www)
/// - reddit.com -> old.reddit.com
#[instrument]
pub async fn updated_reddit(message: &str) -> Option<String> {
    if !REDDIT_PATTERN.is_match(message) {
        return None;
    }

    let updated = REDDIT_PATTERN.replace_all(message, |caps: &Captures| {
        let path = caps.name("path").map_or("", |m| m.as_str());

        if caps.name("m").is_some() {
            // Drop "m." → old.reddit.com
            format!("{REDDIT_REPLACEMENT}.com{path}")
        } else if caps.name("www").is_some() {
            // Keep "www."
            format!("www.{REDDIT_REPLACEMENT}.com{path}")
        } else {
            format!("{REDDIT_REPLACEMENT}.com{path}")
        }
    });

    Some(updated.into_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_no_reddit_url() {
        let message = "This is a regular message without URLs.";
        assert_eq!(updated_reddit(message).await, None);
    }

    #[tokio::test]
    async fn test_reddit_url_with_www() {
        let message = "Check https://www.reddit.com/r/rust";
        let expected = "Check https://www.old.reddit.com/r/rust";
        assert_eq!(updated_reddit(message).await, Some(expected.to_string()));
    }

    #[tokio::test]
    async fn test_reddit_url_without_www() {
        let message = "Visit https://reddit.com/r/programming";
        let expected = "Visit https://old.reddit.com/r/programming";
        assert_eq!(updated_reddit(message).await, Some(expected.to_string()));
    }

    #[tokio::test]
    async fn test_mobile_reddit_url() {
        let message = "Mobile: https://m.reddit.com/r/learnrust";
        let expected = "Mobile: https://old.reddit.com/r/learnrust"; // drop m.
        assert_eq!(updated_reddit(message).await, Some(expected.to_string()));
    }

    #[tokio::test]
    async fn test_reddit_url_with_user() {
        let message = "User page: https://reddit.com/u/username";
        let expected = "User page: https://old.reddit.com/u/username";
        assert_eq!(updated_reddit(message).await, Some(expected.to_string()));
    }

    #[tokio::test]
    async fn test_http_reddit_url() {
        let message = "HTTP version: http://www.reddit.com/r/test";
        let expected = "HTTP version: http://www.old.reddit.com/r/test";
        assert_eq!(updated_reddit(message).await, Some(expected.to_string()));
    }

    #[tokio::test]
    async fn test_multiple_replacements_in_message() {
        let message = "First https://reddit.com/r/a and second https://www.reddit.com/r/b and mobile https://m.reddit.com/r/c";
        let expected = "First https://old.reddit.com/r/a and second https://www.old.reddit.com/r/b and mobile https://old.reddit.com/r/c";
        assert_eq!(updated_reddit(message).await, Some(expected.to_string()));
    }

    #[tokio::test]
    async fn test_reddit_in_text_but_not_url() {
        let message = "I love reddit but not the URL.";
        // This should not match since it's not a URL
        assert_eq!(updated_reddit(message).await, None);
    }
}
