#![warn(clippy::pedantic)]

use std::{convert::Infallible, env, str::FromStr};
use teloxide::{
    update_listeners::{webhooks, UpdateListener},
    Bot,
};
use tracing::info;
use url::Url;

mod telemetry;

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

#[tokio::main]
async fn main() {
    telemetry::init();
    let (bot, listener) = set_up_bot().await;
    Box::pin(deficiente_telegram_bot::parse_messages(bot, listener)).await;
}
