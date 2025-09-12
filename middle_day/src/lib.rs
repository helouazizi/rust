use chrono::{NaiveDate,Datelike, Weekday};

pub fn middle_day(year: u32) -> Option<Weekday> {
    let year_i32 = year as i32;

    if is_leap_year(year_i32) {
        return None;
    }

    let middle_day = (365 + 1) / 2;

    NaiveDate::from_yo_opt(year_i32, middle_day).map(|date| date.weekday())
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}
