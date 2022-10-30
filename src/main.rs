// mod telegram_message;
// mod messages;
use deficientebot_telegram::messages::telegram_message;
fn main() {
    let message = telegram_message::Message {
        content: String::from("uwu"),
    };
    message.defiente_checker();
    message.deficiente();
    // telegram_message::run();
}
