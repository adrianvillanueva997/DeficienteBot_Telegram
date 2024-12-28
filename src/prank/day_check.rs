use chrono::Utc;

use chrono::prelude::*;

#[must_use]
pub fn check_28_december() -> bool {
    let now = Utc::now();
    now.month() == 12 && now.day() == 28
}
