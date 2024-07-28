#[cfg(test)]
mod tests {
    use deficiente_telegram_bot::message_checks::tiktok::{check_if_tiktok, updated_tiktok};

    #[tokio::test]
    async fn test_updated_tiktok() {
        let message = "Check out this link: https://www.tiktok.com";
        let expected = "Check out this link: https://www.tnktok.com";
        assert_eq!(updated_tiktok(message).await, Some(expected.to_string()));
    }

    #[test]
    fn test_check_if_tiktok() {
        let message = "Check out this link: https://www.tiktok.com";
        assert!(check_if_tiktok(message));
    }

    #[test]
    fn test_check_if_tiktok_false() {
        let message = "Check out this link: https://www.example.com";
        assert!(!check_if_tiktok(message));
    }
}
