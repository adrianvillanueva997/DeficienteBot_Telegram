use chrono::Datelike;
use rand::Rng;

static THURSDAY_GREETINGS: &'static [&str] = &[
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

pub async fn is_thursday() -> bool {
    let now = chrono::offset::Local::now();
    if now.date_naive().weekday() == chrono::Weekday::Thu {
        return true;
    }
    false
}

pub async fn check_asuka(message: &str) -> bool {
    message.contains("gracias asuka")
}

pub async fn random_message() -> String {
    let rng = rand::thread_rng().gen_range(0..THURSDAY_GREETINGS.len());
    THURSDAY_GREETINGS[rng].to_string()
}
