#[cfg(test)]
mod tests {
    const THURSDAY_GREETINGS: [&str; 10] = [
        "De nada maquina, que tengas un feliz jueves",
        "De nada mi amor",
        "De nada fiera",
        "Es un placer bb",
        "Ningún problema, para eso estamos las waifus",
        "Que Dios te bendiga :v",
        "Un placer, que tengas un bonito jueves precioso",
        "uwu :3 :v <3",
        "Disfruta del jueves, nos vemos la semana que viene, fiera",
        "las que tu tienes, fiera, crack, maquina, mastodonte",
    ];

    use chrono::Datelike;
    use deficiente_telegram_bot::message_checks::thursday::{
        check_asuka, is_thursday, random_message,
    };

    #[tokio::test]
    async fn test_is_thursday() {
        if chrono::offset::Local::now().date_naive().weekday() == chrono::Weekday::Thu {
            assert!(is_thursday().await);
        }
    }
    #[tokio::test]
    async fn test_check_asuka() {
        let message = "gracias asuka";
        assert!(check_asuka(message).await);
    }
    #[tokio::test]
    async fn test_random_message() {
        let message = random_message().await;
        assert!(THURSDAY_GREETINGS.contains(&message.as_str()));
    }
}
