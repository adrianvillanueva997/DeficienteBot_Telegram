#[cfg(test)]
mod tests {
    use deficiente_telegram_bot::message_checks::copypasta::find_copypasta;

    #[tokio::test]
    async fn test_find_copypasta() {
        let copypasta = "amiga";
        let (_, copypastas) = find_copypasta(copypasta).await;
        assert_eq!(copypastas, (vec!["\"amiga\"".to_string()]));
    }
}
