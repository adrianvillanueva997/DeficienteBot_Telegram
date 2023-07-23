use url::Url;

/// Checks if the message contains a valid url.
pub fn is_url(url: &str) -> bool {
    Url::parse(url).is_ok()
}

// test
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_url() {
        let url = "https://google.com";
        assert!(is_url(url));
    }
}
