/// Checks if the message contains a twitter link and if it does, it replaces it with a link to the same tweet but in fx.
pub async fn update_vxtwitter(message: &str) -> Option<String> {
    if (message.contains("twitter") && message.contains("status"))
        && !(message.contains("fxtwitter") || message.contains("vxtwitter"))
    {
        return Some(message.replace("twitter", "fxtwitter"));
    } else if message.contains("x.com") && message.contains("status") {
        return Some(message.replace("x.com", "fixupx.com"));
    }

    None
}
