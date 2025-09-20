use std::collections::HashMap;
use chrono::{DateTime, Utc, Datelike};
pub fn commits_per_week(data: &json::JsonValue) -> HashMap<String, u32> {
    let mut res: HashMap<String, u32> = HashMap::new();

    for obj in data.members() {
        if let Some(date_str) = obj["commit"]["author"]["date"].as_str() {
            if let Ok(dt) = DateTime::parse_from_rfc3339(date_str) {
                let dt_utc: DateTime<Utc> = dt.with_timezone(&Utc);
                let iso_week = dt_utc.iso_week();
                let key = format!("{}-W{:02}", iso_week.year(), iso_week.week());

                *res.entry(key).or_insert(0) += 1;
            }
        }
    }

    res
}

pub fn commits_per_author(data: &json::JsonValue) -> HashMap<String, u32> {
    let mut res: HashMap<String, u32> = HashMap::new();
    for obj in data.members() {
        if let Some(author_name) = obj["author"]["login"].as_str() {
            *res.entry(author_name.to_string()).or_insert(0) += 1;
        }
    }
   res
}
