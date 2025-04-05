use regex::Regex;
use std::sync::LazyLock;
use tracing::instrument;
const BAD_WORDS: &str = r"\W*((?i)(uwu|:v|owo)(?-i))\W*";

static BAD_WORD_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(BAD_WORDS).unwrap());
/// Find bad words in a string.
///
/// # Panics
///
/// Panics if the regex fails to compile.

#[instrument]
pub async fn find_bad_words(input: &str) -> bool {
    BAD_WORD_REGEX.is_match(input)
}
