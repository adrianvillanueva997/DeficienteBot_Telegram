use chrono::Datelike;

pub fn _get_todays_date() -> (i32, u32, u32) {
    let current_date = chrono::Utc::now().date();
    (
        current_date.year(),
        current_date.month(),
        current_date.day(),
    )
}
