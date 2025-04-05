use teloxide::types::Message;

pub fn escape_markdown(text: &str) -> String {
    let special_chars = [
        '_', '*', '[', ']', '(', ')', '~', '`', '>', '#', '+', '-', '=', '|', '{', '}', '.', '!',
    ];
    let mut result = String::with_capacity(text.len() * 2);
    for c in text.chars() {
        if special_chars.contains(&c) {
            result.push('\\');
        }
        result.push(c);
    }
    result
}

pub fn get_telegram_username(msg: &Message) -> String {
    let user = msg.from.as_ref().unwrap().username.as_ref().unwrap();
    format!("@{user}")
}

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
