use tracing::instrument;
const TNTOK: &str = "tnktok";

/// Checks if the message contains a tiktok url and if it does, it replaces "tiktok" with "tnktok"
#[instrument]
pub async fn updated_tiktok(message: &str) -> Option<String> {
    if message.contains("tiktok") {
        let updated_message = message.replace("tiktok", TNTOK);
        return Some(updated_message);
    }
    None
}

/// Checks if the message contains the word "tiktok"
#[must_use]
#[allow(clippy::module_name_repetitions)]
pub fn check_if_tiktok(message: &str) -> bool {
    message.contains("tiktok")
}
