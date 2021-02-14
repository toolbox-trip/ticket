use chrono::{Duration, Utc};

pub fn get_date(offset: i64) -> String {
    let now = Utc::now() + Duration::days(offset);
    now.format("%Y-%m-%d").to_string()
}
