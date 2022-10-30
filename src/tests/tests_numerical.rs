// The next line exists to trick play.rust-lang.org into running our code as a
// test:
// fn main
#[cfg(test)]
pub mod tests {
    use crate::checks::telegram_message;

    #[test]
    fn test_numerical_1() {
        let message = telegram_message::Checkings {
            content: String::from("hola chiquis 420"),
        };
        let result = message.numerical_checks();
        assert!(result);
    }
    #[test]
    fn test_numerical_2() {
        let message = telegram_message::Checkings {
            content: String::from("akmfamfafd420asodaofoaf"),
        };
        let result = message.numerical_checks();
        assert!(result);
    }

    #[test]

    fn test_numerical_3() {
        let message = telegram_message::Checkings {
            content: String::from("i like 69"),
        };
        let result = message.numerical_checks();
        assert!(result);
    }
    #[test]
    fn test_numerical_4() {
        let message = telegram_message::Checkings {
            content: String::from("asfafssafa69afafnajsnfa"),
        };
        let result = message.numerical_checks();
        assert!(result);
    }
}
