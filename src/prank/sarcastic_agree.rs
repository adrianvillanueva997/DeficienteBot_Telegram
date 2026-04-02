use rand::seq::IndexedRandom;

static SARCASTIC_REPLIES: &[&str] = &[
    "Ostia qué crack, cómo no se nos había ocurrido antes",
    "Wow, qué aportación más increíble, gilipollas 👏👏👏",
    "Gracias por iluminarnos con tu sabiduría",
    "Esto es lo más inteligente que he leído hoy, y eso que he leído poco",
    "Joder tío, qué profundo, me has cambiado la vida",
    "Increíble reflexión, chaval, de verdad",
    "Toma upvote imaginario, 🔼",
    "Brutal, compártelo en LinkedIn",
];

#[must_use]
pub fn random_sarcastic_reply() -> &'static str {
    SARCASTIC_REPLIES
        .choose(&mut rand::rng())
        .unwrap_or(&"Tienes razón, deficiente 👍")
}
