#[cfg(test)]
mod tests {
    use deficiente_telegram_bot::message_checks::webm::{
        self, check_url_status_code, convert_webm_to_mp4, delete_mp4, delete_webm, download_webm,
        mp4_exists, url_is_webm, webm_exists,
    };
    use uuid::Uuid;

    const URL :&str = "https://cdn.discordapp.com/attachments/851166737692098580/933357925852921886/Never_Ending_Tech.webm";
    #[tokio::test]
    async fn test_check_url_status_code() {
        let url = "https://google.com";
        assert_eq!(check_url_status_code(url).await, Some(200));
    }
    #[test]
    fn test_url_is_webm() {
        assert!(url_is_webm(URL));
    }
    #[tokio::test]
    async fn test_e2e_webm_workflow() {
        let uuid = Uuid::new_v4();
        let webm_filename = format!("{}.webm", uuid);
        let mp4_filename = format!("{}.mp4", uuid);
        download_webm(URL, &webm_filename).await;
        convert_webm_to_mp4(&webm_filename, &mp4_filename).await;
        assert!(mp4_exists(&mp4_filename).await);
        delete_webm(&webm_filename).await;
        delete_mp4(&mp4_filename).await;
        assert!(!webm_exists(&webm_filename).await);
        assert!(!mp4_exists(&mp4_filename).await);
    }
}
