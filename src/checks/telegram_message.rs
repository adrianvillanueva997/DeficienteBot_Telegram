use lazy_static::lazy_static;
use regex::RegexSet;
#[derive(Debug)]
pub struct Checkings {
    pub content: String,
}

impl Checkings {
    pub fn build(text: String) -> Checkings {
        Checkings { content: text }
    }
    /// Returns the is url of this [`Message`].
    fn is_url(&self) -> bool {
        self.content.contains("twitter") && self.content.contains("status")
    }

    /// Returns the vx twitter of this [`Message`].
    pub fn vx_twitter(&self) -> String {
        if self.is_url() {
            return self.content.replace("twitter", "vxtwitter");
        }
        String::from("")
    }

    pub fn deficiente(&self) -> bool {
        lazy_static! {
            static ref RE: RegexSet = RegexSet::new(&[r"uwu", r":v", r"owo"]).unwrap();
        }
        RE.is_match(&self.content)
    }
    fn numerical_regex(&self, text: &str) -> bool {
        lazy_static! {
            static ref RE: RegexSet = RegexSet::new(&[r"69", r"420"]).unwrap();
        }
        RE.is_match(text)
    }

    pub fn numerical_checks(&self) -> bool {
        self.numerical_regex(&self.content)
    }
}
