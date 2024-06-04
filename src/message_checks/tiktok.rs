use lazy_static::lazy_static;
use regex::Regex;
use tracing::instrument; // Add import for lazy_static
const TNTOK: &str = "tnktok";

lazy_static! {
    static ref RE: Regex = Regex::new(r"https://(www\.)?tiktok\.com/.*").unwrap();
}

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

/// .
#[must_use]
pub fn check_if_tiktok(message: &str) -> bool {
    RE.is_match(message)
}
