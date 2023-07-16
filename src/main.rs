use log::info;
use std::{env, str::FromStr};
use teloxide::{update_listeners::webhooks, Bot};
use url::Url;

// use crate::routines::{birthdays::birthday_routine, thursday::happy_thursday_routine};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let bot: Bot = Bot::from_env();
    let addr = ([0, 0, 0, 0], 80).into();
    let url = env::var("url").expect("URL is not set");
    let url = Url::from_str(&url).unwrap();
    let listener = webhooks::axum(bot.clone(), webhooks::Options::new(addr, url.clone()))
        .await
        .expect("Couldn't setup webhook");
    info!("Running on {} {}", url, addr);

    webhookstuff::parse_messages(bot, listener).await;
}
