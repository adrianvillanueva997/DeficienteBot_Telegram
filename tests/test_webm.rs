#[cfg(test)]
mod tests {
    use deficiente_telegram_bot::message_checks::webm::{
        check_url_status_code, convert_webm_to_mp4, delete_mp4, delete_webm, download_webm,
        is_webm_url, mp4_exists, webm_exists,
    };
    use uuid::Uuid;

    const URL: &str = "https://i.4cdn.org/wsg/1708205885082829.webm";
    #[tokio::test]
    async fn test_check_url_status_code() {
        let url = "https://google.com";
        assert_eq!(check_url_status_code(url).await, Some(200));
    }
    #[test]
    fn test_url_is_webm() {
        assert!(is_webm_url(URL));
    }
    #[tokio::test]
    async fn test_e2e_webm_workflow() {
        let uuid = Uuid::new_v4();
        let webm_filename = format!("{}.webm", uuid);
        let mp4_filename = format!("{}.mp4", uuid);
        println!("webm_filename: {}", webm_filename);
        println!("mp4_filename: {}", mp4_filename);
        download_webm(URL, &webm_filename).await;
        convert_webm_to_mp4(&webm_filename, &mp4_filename).await;
        assert!(mp4_exists(&mp4_filename).await);
        delete_webm(&webm_filename).await;
        delete_mp4(&mp4_filename).await;
        assert!(!webm_exists(&webm_filename).await);
        assert!(!mp4_exists(&mp4_filename).await);
    }
}
