use chrono::Utc;

pub fn get_date() -> String {
    let now = Utc::now();
    now.format("%Y-%m-%d").to_string()
}
