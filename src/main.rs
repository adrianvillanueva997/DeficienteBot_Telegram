#![warn(clippy::pedantic)]

use std::{convert::Infallible, env, str::FromStr};
use teloxide::{
    update_listeners::{webhooks, UpdateListener},
    Bot,
};
use tracing::info;
use url::Url;

async fn set_up_bot() -> (Bot, impl UpdateListener<Err = Infallible>) {
    let bot: Bot = Bot::from_env();
    let addr = ([0, 0, 0, 0], 8080).into();
    let url = env::var("URL").expect("URL is not set");
    let url = Url::from_str(&url).unwrap();
    let listener = webhooks::axum(bot.clone(), webhooks::Options::new(addr, url.clone()))
        .await
        .expect("Couldn't setup webhook");
    info!("Running on {} {}", url, addr);
    (bot, listener)
}

/// Main function.
///
/// # Panics
///
/// Panics if the url and bot token are not set.
#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    color_eyre::install().unwrap();
    // env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let (bot, listener) = set_up_bot().await;
    Box::pin(deficiente_telegram_bot::parse_messages(bot, listener)).await;
}
// redis url = redis://localhost:6379
