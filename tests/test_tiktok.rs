#[cfg(test)]
mod tests {
    use deficiente_telegram_bot::message_checks::tiktok::{check_if_tiktok, updated_tiktok};

    #[tokio::test]
    async fn test_check_if_tiktok_with_tiktok_link() {
        let message =
            "Check out this tiktok link: https://www.tiktok.com/@username/video/123456789";
        assert!(check_if_tiktok(message));
    }

    #[tokio::test]
    async fn test_check_if_tiktok_without_tiktok_link() {
        let message = "This is a regular message without any tiktok link";
        assert!(!check_if_tiktok(message));
    }

    #[tokio::test]
    async fn test_updated_tiktok_with_tiktok_link() {
        let message =
            "Check out this tiktok link: https://www.tiktok.com/@username/video/123456789";
        let expected_result = Some("Check out this tiktok link: tnktok".to_string());
        assert_eq!(updated_tiktok(message).await, expected_result);
    }

    #[tokio::test]
    async fn test_updated_tiktok_without_tiktok_link() {
        let message = "This is a regular message without any tiktok link";
        let expected_result = None;
        assert_eq!(updated_tiktok(message).await, expected_result);
    }

    // Additional tests

    #[tokio::test]
    async fn test_check_if_tiktok_with_vm_tiktok_link() {
        let message = "Check out this tiktok link: https://vm.tiktok.com/@username/video/123456789";
        assert!(check_if_tiktok(message));
    }

    #[tokio::test]
    async fn test_updated_tiktok_with_vm_tiktok_link() {
        let message = "Check out this tiktok link: https://vm.tiktok.com/@username/video/123456789";
        let expected_result = Some("Check out this tiktok link: tnktok".to_string());
        assert_eq!(updated_tiktok(message).await, expected_result);
    }
}
