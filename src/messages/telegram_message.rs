use lazy_static::lazy_static;
use regex::RegexSet;
#[derive(Debug)]
pub struct Message {
    pub content: String,
}

impl Message {
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

    fn deficiente_regex(&self, text: &str) -> bool {
        lazy_static! {
            static ref RE: RegexSet = RegexSet::new(&[r"uwu", r":v"]).unwrap();
        }
        RE.is_match(text)
    }

    pub fn deficiente(&self) -> bool {
        let is_deficiente = self.deficiente_regex(&self.content);
        println!("{is_deficiente}",);
        is_deficiente
    }
}
