use lazy_static::lazy_static;
use regex::Regex;
use tokio::task;

const BAD_WORDS: &str = r#"\W*((?i)(uwu|:v|:3|owo)(?-i))\W*"#;

lazy_static! {
    static ref BAD_WORD_REGEX: Regex = Regex::new(BAD_WORDS).unwrap();
}
pub async fn find_bad_words(input: &str) -> bool {
    let input = input.to_string();
    // BAD_WORD_REGEX.is_match(input)
    task::spawn_blocking(move || BAD_WORD_REGEX.is_match(&input))
        .await
        .unwrap()
}
