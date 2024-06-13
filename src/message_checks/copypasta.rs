use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use tracing::instrument;

const WORDS: [&str; 12] = [
    "cbt",
    "lentejas",
    "amiga",
    "halloween",
    "ayuso",
    "bicho",
    "amogus",
    "china",
    "euskadi",
    "69",
    "viernes",
    "amogus",
];

/// Find matching words in a string.
#[instrument]
pub async fn find_matching_words(sentence: &str) -> Vec<String> {
    let mut matching_words = Vec::new();
    let split_sentence = sentence.split_whitespace().collect::<Vec<_>>();
    for token in split_sentence {
        for word in WORDS {
            if token.to_lowercase().contains(word) {
                matching_words.push(word.to_string());
            }
        }
    }

    matching_words
}

/// Finds copypastas in a string
#[instrument]
pub async fn find_copypasta(input: &str) -> (Vec<String>, Vec<String>) {
    let words = find_matching_words(input).await;
    let copypastas = words
        .par_iter()
        .map(|word| copypastas(word).to_string())
        .collect();
    (words, copypastas)
}

/// Return the copypasta for a given word.
fn copypastas(word: &str) -> &str {
    match word {
        "cbt" => "Cock and ball torture (CBT), penis torture or dick torture is a sexual activity involving application of pain or constriction to the penis or testicles. This may involve directly painful activities, such as genital piercing, wax play, genital spanking, squeezing, ball-busting, genital flogging, urethral play, tickle torture, erotic electrostimulation, kneeing or kicking.",
        "lentejas" => "Lo mejor de las maduras, es que puedes comerles el roscón de reyes, mientras te tienen al fuego unas lentejas de puta madre. Yo recuerdo una que conocí en un eroski, y la recuerdo como uno de los mejores polvos de mi vida. Ella me hizo unos callos cojonudisisimos, y mientras los preparaba, yo le daba como un cabrón por el ojete, ya que se había puesto faldita para que fuese haciendo mientras cocinaba. Creo que eyaculé tal cantidad de esperma, que estuve dos horas inconsciente. Menos mal que los callos me dieron fuerza para acabar el día con un par más. Y tenía unos hijos majísimos. Menudos vicios echamos al crash bandicoot 2.",
        "ayuso" => "Estoy muy jodido, Ayuso me ha arruinado la puta vida, cuando me despierto lo primero que veo es un cartel de Ayuso colgado en mi pared que me pone muy cachondo pero tengo que prepararme para ir al trabajo no sin antes ir al baño para hacerme la paja con Ayuso. En trabajo voy cada dos horas a hacerme la paja con Ayuso y mientras estoy trabajando solo pienso en ella y en lo guapa que es. Cuando por fin llego a mi casa lo primero que hago es hacerme una paja con Ayuso, después me ducho ceno y me vuelvo a hacer una paja con Ayuso, luego voy a ver los mejores discursos de Ayuso y mientras me pongo cachondo al final del día me voy a la cama y me hago un par de pajas más con Ayuso y después me acuesto y me despierto a media noche así que me hago otra paja. En los fin de semanas me levanto con un charco de semen en le pantalón solo de pensar en Ayuso así que me hago otra paja con Ayuso y voy a ducharme, como y me hago una paja con Ayuso, desayuno y me hago una paja con Ayuso, ando y me hago una paja con Ayuso pajas pajas Ayuso pajas Ayuso",
        "bicho" => "SIIUUUUUUUUUUUUU",
        "china" => "
⣿⣿⣿⣿⣿⠟⠋⠄⠄⠄⠄⠄⠄⠄⢁⠈⢻⢿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⠃⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠈⡀⠭⢿⣿⣿⣿⣿
⣿⣿⣿⣿⡟⠄⢀⣾⣿⣿⣿⣷⣶⣿⣷⣶⣶⡆⠄⠄⠄⣿⣿⣿⣿
⣿⣿⣿⣿⡇⢀⣼⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣧⠄⠄⢸⣿⣿⣿⣿
⣿⣿⣿⣿⣇⣼⣿⣿⠿⠶⠙⣿⡟⠡⣴⣿⣽⣿⣧⠄⢸⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣾⣿⣿⣟⣭⣾⣿⣷⣶⣶⣴⣶⣿⣿⢄⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⡟⣩⣿⣿⣿⡏⢻⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣹⡋⠘⠷⣦⣀⣠⡶⠁⠈⠁⠄⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣍⠃⣴⣶⡔⠒⠄⣠⢀⠄⠄⠄⡨⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣦⡘⠿⣷⣿⠿⠟⠃⠄⠄⣠⡇⠈⠻⣿⣿⣿⣿
⣿⣿⣿⣿⡿⠟⠋⢁⣷⣠⠄⠄⠄⠄⣀⣠⣾⡟⠄⠄⠄⠄⠉⠙⠻
⡿⠟⠋⠁⠄⠄⠄⢸⣿⣿⡯⢓⣴⣾⣿⣿⡟⠄⠄⠄⠄⠄⠄⠄⠄
⠄⠄⠄⠄⠄⠄⠄⣿⡟⣷⠄⠹⣿⣿⣿⡿⠁⠄⠄⠄⠄⠄⠄⠄⠄
为党争光! Glory to the CCP!",
        "halloween" => "Qué es Jalogüin??",
        "amiga" => "\"amiga\"",
        "euskadi" => "el ojo de",
        "69" => "> nice",
        "viernes" => "viernes",
        "amogus" => "ඞ",
        _ => ""

    }
}
