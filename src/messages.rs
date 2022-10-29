#[derive(Debug)]
pub struct Message {
    pub content: String,
}

impl Message {
    fn is_url(&self) -> bool {
        self.content.contains("twitter") && self.content.contains("status")
    }

    pub fn vx_twitter(&self) -> String {
        if self.is_url() {
            return self.content.replace("twitter", "vxtwitter");
        }
        String::from("")
    }
}
