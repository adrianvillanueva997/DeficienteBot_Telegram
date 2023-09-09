#[cfg(test)]
mod tests {
    use deficiente_telegram_bot::message_checks::url::is_url;

    #[test]
    fn test_is_url() {
        let url = "https://google.com";
        assert!(is_url(url));
    }
}
