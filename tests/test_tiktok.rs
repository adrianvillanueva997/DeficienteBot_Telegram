#[cfg(test)]
mod tests {
    use deficiente_telegram_bot::message_checks::tiktok::updated_tiktok;

    #[tokio::test]
    async fn test_update_vmtiktok() {
        let tiktok = "https://tiktok.com/ZGehAaDPK/";
        let tntok = "https://tnktok.com/ZGehAaDPK/";
        assert_eq!(updated_tiktok(tiktok).await, Some(tntok.to_string()));
    }
    #[tokio::test]
    async fn test_update_tiktok() {
        let tiktok = "https://vm.tiktok.com/ZGehAaDPK/";
        let tntok = "https://tnktok.com/ZGehAaDPK/";
        assert_eq!(updated_tiktok(tiktok).await, Some(tntok.to_string()));
    }
}
