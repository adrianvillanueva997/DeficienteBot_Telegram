use teloxide::types::Message;

pub fn format_message_username(msg: &Message, content: &str) -> String {
    let username = msg
        .from
        .as_ref()
        .and_then(|user| user.username.as_ref())
        .map_or_else(
            || "👤 *Anonymous User*".to_string(),
            |name| format!("👤 @{name} sent:"),
        );
    format!("{username}\n\n{content}")
}
