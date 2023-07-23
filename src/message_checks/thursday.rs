use chrono::Datelike;
use rand::Rng;

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

/// Check if today is Thursday.
pub async fn is_thursday() -> bool {
    let now = chrono::offset::Local::now();
    if now.date_naive().weekday() == chrono::Weekday::Thu {
        return true;
    }
    false
}

/// Check if the message contains "gracias asuka".
pub async fn check_asuka(message: &str) -> bool {
    message.contains("gracias asuka")
}

/// Return a random message from the THURSDAY_GREETINGS array.
pub async fn random_message() -> String {
    let rng = rand::thread_rng().gen_range(0..THURSDAY_GREETINGS.len());
    THURSDAY_GREETINGS[rng].to_string()
}

//test
#[cfg(test)]
mod tests {
    use super::*;
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
