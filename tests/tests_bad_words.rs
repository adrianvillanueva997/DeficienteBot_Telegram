mod tests {
    use deficiente_telegram_bot::message_checks::bad_words::find_bad_words;

    #[tokio::test]
    async fn test_uwu_badword() {
        let bad_words = "uwu";
        assert!(find_bad_words(bad_words).await);
    }
    #[tokio::test]
    async fn test_v_badword() {
        let bad_words = ":v";
        assert!(find_bad_words(bad_words).await);
    }
    #[tokio::test]
    async fn test_owo_badword() {
        let bad_words = "owo";
        assert!(find_bad_words(bad_words).await);
    }
    #[tokio::test]
    async fn test_no_badword() {
        let bad_words = "hola";
        assert!(!find_bad_words(bad_words).await);
    }
}
