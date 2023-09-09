#[cfg(test)]
mod tests {
    use deficiente_telegram_bot::message_checks::copypasta::find_copypasta;

    #[tokio::test]
    async fn test_find_copypasta() {
        let copypasta = "amiga";
        assert_eq!(
            find_copypasta(copypasta).await,
            vec!["\"amiga\"".to_string()]
        );
    }
}
