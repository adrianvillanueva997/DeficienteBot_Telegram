use tracing::instrument;

const TNTOK: &str = "tnktok";

/// Checks if the message contains a tiktok link and if it does, it replaces it with a link to tntok (a privacy focused version of tiktok)
#[instrument]
pub async fn updated_tiktok(message: &str) -> Option<String> {
    if message.contains("https://vm.tiktok") {
        return Some(message.replace("vm.tiktok", TNTOK));
    } else if message.contains("https://tiktok.com/") {
        return Some(message.replace("tiktok", TNTOK));
    }
    None
}

pub async fn is_tiktok(message: &str) -> bool {
    message.contains("https://vm.tiktok") || message.contains("https://tiktok.com/")
}
