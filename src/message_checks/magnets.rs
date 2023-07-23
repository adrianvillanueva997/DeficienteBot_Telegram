use lazy_static::lazy_static;
use regex::Regex;
use tokio::io::AsyncWriteExt;
lazy_static! {
    static ref MAGNET_REGEX: Regex = Regex::new(r"magnet:\?xt=urn:btih:[a-zA-Z0-9]{40}").unwrap();
}

pub const TORRENT_FILE: &str = "torrent.torrent";

pub async fn is_magnet(msg: &str) -> bool {
    let caps = MAGNET_REGEX.captures(msg);
    if caps.is_some() {
        return true;
    }
    false
}

pub async fn torrent_exists() {
    if std::path::Path::new(TORRENT_FILE).exists() {
        delete_torrent().await;
    }
}

async fn delete_torrent() {
    std::fs::remove_file(TORRENT_FILE).expect("Failed to delete torrent");
}

pub async fn write_magnet_to_file(magnet: &str) {
    let mut file = tokio::fs::File::create(TORRENT_FILE)
        .await
        .expect("Failed to create file");
    file.write_all(magnet.as_bytes())
        .await
        .expect("Failed to write to file");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_is_magnet() {
        let magnet = "magnet:?xt=urn:btih:0a9c8e1b7a7b6e1b7a7b6e1b7a7b6e1b7a7b6e1b";
        assert!(is_magnet(magnet).await);
    }
}
