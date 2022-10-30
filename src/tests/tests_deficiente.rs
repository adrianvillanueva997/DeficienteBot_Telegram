// The next line exists to trick play.rust-lang.org into running our code as a
// test:
// fn main
#[cfg(test)]
pub mod tests {
    use crate::messages::telegram_message;

    #[test]
    fn test_isdeficiente_1() {
        let message = telegram_message::Message {
            content: String::from("hola chiquis uwu"),
        };
        let result = message.deficiente();
        assert!(result);
    }
    #[test]

    fn test_isdeficiente_2() {
        let message = telegram_message::Message {
            content: String::from("hola amigos de youtube"),
        };
        let result = message.deficiente();
        assert!(!result);
    }
    #[test]

    fn test_isdeficiente_3() {
        let message = telegram_message::Message {
            content: String::from("hola amigos :v"),
        };
        let result = message.deficiente();
        assert!(result);
    }
}
