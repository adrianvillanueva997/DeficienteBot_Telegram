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

// test
#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_update_vxtwitter() {
        let twitter = "https://twitter.com/AsukaLangleyS/status/1375160000000000000";
        let fxtwitter = "https://fxtwitter.com/AsukaLangleyS/status/1375160000000000000";
        assert_eq!(update_vxtwitter(twitter).await, Some(fxtwitter.to_string()));
    }
    #[tokio::test]
    async fn test_update_xcom() {
        let xcom = "https://x.com/AsukaLangleyS/status/1375160000000000000";
        let fixupx = "https://fixupx.com/AsukaLangleyS/status/1375160000000000000";
        assert_eq!(update_vxtwitter(xcom).await, Some(fixupx.to_string()));
    }
}
