use chrono::Datelike;
use rand::Rng;
use tracing::instrument;

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

const ASUKA: &str = "gracias asuka";

/// Check if today is Thursday.
#[instrument]
pub async fn is_thursday() -> bool {
    let now = chrono::offset::Local::now();
    now.date_naive().weekday() == chrono::Weekday::Thu
}

/// Check if the message contains "gracias asuka"
#[instrument]
pub async fn check_asuka(message: &str) -> bool {
    message.contains(ASUKA)
}

/// Return a random message from the THURSDAY_GREETINGS array.
#[instrument]
pub async fn random_message() -> String {
    let rng = rand::rng().random_range(0..THURSDAY_GREETINGS.len());
    THURSDAY_GREETINGS[rng].to_string()
}
