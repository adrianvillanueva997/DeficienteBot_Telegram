// The next line exists to trick play.rust-lang.org into running our code as a
// test:
// fn main
mod messages;
#[cfg(test)]
mod tests {
    use crate::messages;

    /// Checks if the tweet url is transformed correctly.
    #[test]
    fn test_vxtwitter_1() {
        let message = messages::Message {
            content: String::from("https://twitter.com/timClicks/status/1586130960151900160"),
        };
        let result = message.vx_twitter();
        assert_eq!(
            result,
            "https://vxtwitter.com/timClicks/status/1586130960151900160"
        );
    }
    /// Checks if a normal tweet user is ignored correctly.
    #[test]
    fn test_vxtwitter_2() {
        let message = messages::Message {
            content: String::from("https://twitter.com/timClicks"),
        };
        let result = message.vx_twitter();
        assert_eq!(result, "");
    }
}
