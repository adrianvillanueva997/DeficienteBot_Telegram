use lazy_static::lazy_static;
use regex::Regex;
use tracing::instrument; // Add import for lazy_static
const TNTOK: &str = "tnktok";

lazy_static! {
    static ref RE: Regex = Regex::new(r"https://(www\.|vm\.)?tiktok\.com/.*").unwrap();
}

/// Checks if the message contains a tiktok link and if it does, it replaces it with a link to tntok (a privacy focused version of tiktok)
#[instrument]
pub async fn updated_tiktok(message: &str) -> Option<String> {
    if RE.is_match(message) {
        let updated_message = RE.replace_all(message, TNTOK);
        return Some(updated_message.into_owned());
    }
    None
}

/// .
#[must_use]
#[allow(clippy::module_name_repetitions)]
pub fn check_if_tiktok(message: &str) -> bool {
    RE.is_match(message)
}
