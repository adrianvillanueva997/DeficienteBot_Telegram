use lazy_static::lazy_static;
use regex::Regex;
use tokio::task;

const BAD_WORDS: &str = r"\W*((?i)(uwu|:v|owo)(?-i))\W*";

lazy_static! {
    static ref BAD_WORD_REGEX: Regex = Regex::new(BAD_WORDS).unwrap();
}
/// Find bad words in a string.
///
/// # Panics
///
/// Panics if the regex fails to compile.
pub async fn find_bad_words(input: &str) -> bool {
    let input = input.to_string();
    task::spawn_blocking(move || BAD_WORD_REGEX.is_match(&input))
        .await
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_find_bad_words() {
        let bad_words = "uwu";
        assert!(find_bad_words(bad_words).await);
    }
}
