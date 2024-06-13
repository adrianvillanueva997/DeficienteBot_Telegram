// #[cfg(test)]
// mod tests {
//     use deficiente_telegram_bot::{
//         message_checks::webm::{convert_webm_to_mp4, mp4_exists, webm_exists},
//         online_downloads::{
//             url_checker::{check_url_status_code, is_webm_url},
//             video_downloader::{delete_file, download_video},
//         },
//     };
//     use uuid::Uuid;

//     const URL: &str = "https://is2.4chan.org/wsg/1716815519675831.webm";
//     #[tokio::test]
//     async fn test_check_url_status_code() {
//         let url = "https://google.com";
//         assert_eq!(check_url_status_code(url).await, Some(200));
//     }
//     #[test]
//     fn test_url_is_webm() {
//         assert!(is_webm_url(URL));
//     }
//     #[tokio::test]
//     async fn test_e2e_webm_workflow() {
//         let uuid = Uuid::new_v4();
//         let webm_filename = format!("{}.webm", uuid);
//         let mp4_filename = format!("{}.mp4", uuid);
//         println!("webm_filename: {}", webm_filename);
//         println!("mp4_filename: {}", mp4_filename);
//         download_video(URL, &webm_filename).await;
//         convert_webm_to_mp4(&webm_filename, &mp4_filename).await;
//         assert!(mp4_exists(&mp4_filename).await);
//         delete_file(&webm_filename).await;
//         delete_file(&mp4_filename).await;
//         assert!(!webm_exists(&webm_filename).await);
//         assert!(!mp4_exists(&mp4_filename).await);
//     }
// }
