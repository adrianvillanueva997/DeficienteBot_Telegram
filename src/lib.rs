use std::time::Duration;

use log::info;
use routines::birthdays::special_event;
use teloxide::{requests::Requester, Bot};
use tokio::time;

use crate::routines::utils::{calculate_next_midnight, is_thursday};
pub mod checks;
pub mod routines;
mod tests;
use teloxide::types::ChatId;
const GROUP_ID: i64 = -1063900471;
// const DEV_GROUP_ID: i64 = "-281597102";

pub async fn special_events(bot: Bot) {
    info!("Starting thursday routine");
    loop {
        info!("Calculating next midnight");
        let sleep_seconds = calculate_next_midnight();
        time::sleep(Duration::from_secs(sleep_seconds)).await;
        info!("Running special event routine");
        let event = special_event();
        if !event.is_empty() {
            let _ = bot.send_message(ChatId(GROUP_ID), event).await;
        }
    }
}

pub async fn happy_thursday_routine(bot: Bot) {
    info!("Starting thursday routine");
    let mut sent = false;
    loop {
        if is_thursday() && !sent {
            info!("Running thursday routine");
            let _ = bot.send_message(ChatId(GROUP_ID), "Feliz jueves!").await;
            sent = true;
        } else {
            info!("Calculating next midnight");
            let sleep_seconds = calculate_next_midnight();
            time::sleep(Duration::from_secs(sleep_seconds)).await;
            sent = false;
        }
    }
}
