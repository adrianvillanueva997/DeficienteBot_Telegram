pub async fn update_vxtwitter(message: &str) -> Option<String> {
    if (message.contains("twitter") && message.contains("status"))
        && !(message.contains("fxtwitter") || message.contains("vxtwitter"))
    {
        let url = message.replace("twitter", "fxtwitter");
        return Some(url);
    }

    None
}
