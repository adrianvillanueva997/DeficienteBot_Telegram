use chrono::Utc;

use chrono::prelude::*;

#[must_use]
pub fn is_prank_day() -> bool {
    let now = Utc::now();
    now.month() == 12 && now.day() == 28 || now.month() == 4 && now.day() == 1
}
