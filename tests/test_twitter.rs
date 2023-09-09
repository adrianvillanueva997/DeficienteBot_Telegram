#[cfg(test)]
mod tests {
    use deficiente_telegram_bot::message_checks::twitter::update_vxtwitter;

    #[tokio::test]
    async fn test_update_vxtwitter() {
        let twitter = "https://twitter.com/AsukaLangleyS/status/1375160000000000000";
        let fxtwitter = "https://fxtwitter.com/AsukaLangleyS/status/1375160000000000000";
        assert_eq!(update_vxtwitter(twitter).await, Some(fxtwitter.to_string()));
    }
    #[tokio::test]
    async fn test_update_xcom() {
        let xcom = "https://x.com/AsukaLangleyS/status/1375160000000000000";
        let fixupx = "https://fixupx.com/AsukaLangleyS/status/1375160000000000000";
        assert_eq!(update_vxtwitter(xcom).await, Some(fixupx.to_string()));
    }
}
