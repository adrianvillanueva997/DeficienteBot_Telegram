use chrono::{Datelike, Local};
use rand::Rng;

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

#[derive(Debug)]
pub struct ThursdayChecker {}

impl ThursdayChecker {
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }
    fn contains_trigger(message: &str) -> bool {
        message.contains(TRIGGER_PHRASE)
    }
    fn is_thursday() -> bool {
        Local::now().date_naive().weekday() == chrono::Weekday::Thu
    }
    #[must_use]
    pub fn asuka(&self, message: &str) -> Option<&str> {
        if Self::contains_trigger(message) && Self::is_thursday() {
            let rng = rand::rng().random_range(0..THURSDAY_GREETINGS.len());
            return Some(THURSDAY_GREETINGS[rng]);
        }
        None
    }
}

impl Default for ThursdayChecker {
    fn default() -> Self {
        Self::new()
    }
}
