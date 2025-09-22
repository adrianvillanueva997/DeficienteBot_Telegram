use regex::Regex;
use std::sync::LazyLock;
use tracing::instrument;

/// Module for checking and updating Reddit URLs in messages.
///
/// This module provides functionality to detect Reddit URLs and replace them
/// with old.reddit equivalents to avoid modern Reddit's interface.
static REDDIT_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"https?://(?:www\.)?(?:m\.)?reddit\.com(?:/(@?[^/\s]+/?)*)?").unwrap()
});
const REDDIT_REPLACEMENT: &str = "old.reddit";

/// Updates Reddit URLs in a message to use old.reddit.
///
/// This function checks if the message contains a Reddit URL and, if so,
/// replaces "reddit" with "old.reddit" in the URL to redirect to the old Reddit interface.
///
/// # Arguments
/// * `message` - The message string to check and potentially update.
///
/// # Returns
/// * `Some(String)` - The updated message with Reddit URLs modified.
/// * `None` - If no Reddit URL is found in the message.
/// ```
#[instrument]
pub async fn updated_reddit(message: &str) -> Option<String> {
    if REDDIT_PATTERN.is_match(message) {
        Some(message.replace("reddit", REDDIT_REPLACEMENT))
    } else {
        None
    }
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
        let expected = "Mobile: https://m.old.reddit.com/r/learnrust";
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
        let message = "First https://reddit.com/r/a and second https://www.reddit.com/r/b";
        let expected = "First https://old.reddit.com/r/a and second https://www.old.reddit.com/r/b";
        assert_eq!(updated_reddit(message).await, Some(expected.to_string()));
    }

    #[tokio::test]
    async fn test_reddit_in_text_but_not_url() {
        let message = "I love reddit but not the URL.";
        // This should not match since it's not a URL
        assert_eq!(updated_reddit(message).await, None);
    }
}
