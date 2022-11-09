use chrono::{Datelike, Duration, Local};
use log::info;
pub fn get_todays_date() -> (i32, u32, u32) {
    let current_date = chrono::Utc::now().date();
    (
        current_date.year(),
        current_date.month(),
        current_date.day(),
    )
}

pub fn calculate_next_midnight() -> u64 {
    let now = Local::now();
    let tomorrow_midnight = (now + Duration::days(1)).date().and_hms(0, 0, 0);
    let duration = tomorrow_midnight
        .signed_duration_since(now)
        .to_std()
        .unwrap();
    info!(
        "Duration between {:?} and {:?}: {:?}",
        now, tomorrow_midnight, duration
    );
    duration.as_secs()
}

pub fn is_thursday() -> bool {
    let current_time = chrono::offset::Local::now();
    current_time.date().weekday() == chrono::Weekday::Thu
}
