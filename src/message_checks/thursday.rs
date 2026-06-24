use chrono::{Datelike, Local};
use rand::RngExt;

static THURSDAY_GREETINGS: &[&str] = &[
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

const TRIGGER_PHRASE: &str = "gracias asuka";

#[must_use]
pub fn check_thursday(message: &str) -> Option<&'static str> {
    if message.contains(TRIGGER_PHRASE) && Local::now().date_naive().weekday() == chrono::Weekday::Thu
    {
        let rng = rand::rng().random_range(0..THURSDAY_GREETINGS.len());
        return Some(THURSDAY_GREETINGS[rng]);
    }
    None
}
