use regex::Regex;

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

    pub fn defiente_checker(&self) {
        let re = Regex::new(r"\\W*((?i)%s(?-i))\\W*").unwrap();
    }
    /// Returns the deficiente of this [`Message`].
    pub fn deficiente(&self) {}
}
