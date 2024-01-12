use log::info;
use std::{env, str::FromStr};
use teloxide::{update_listeners::webhooks, Bot};
use url::Url;

/// Main function.
///
/// # Panics
///
/// Panics if the url and bot token are not set.
#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let bot: Bot = Bot::from_env();
    let addr = ([0, 0, 0, 0], 8080).into();
    let url = env::var("url").expect("URL is not set");
    let url = Url::from_str(&url).unwrap();
    let listener = webhooks::axum(bot.clone(), webhooks::Options::new(addr, url.clone()))
        .await
        .expect("Couldn't setup webhook");
    info!("Running on {} {}", url, addr);

    deficiente_telegram_bot::parse_messages(bot, listener).await;
}
