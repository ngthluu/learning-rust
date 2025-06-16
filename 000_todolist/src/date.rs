use std::time::{SystemTime, UNIX_EPOCH};

pub fn format_datetime(time: SystemTime) -> Result<String, String> {
    let duration = time
        .duration_since(UNIX_EPOCH)
        .map_err(|_| "Invalid time".to_string())?;

    let timestamp = duration.as_secs();
    let (year, month, day) = unix_timestamp_to_date(timestamp);
    let seconds_in_day = timestamp % 86_400;

    let hour = (seconds_in_day / 3600) as u32;
    let minute = (seconds_in_day % 3600) / 60;
    let second = seconds_in_day % 60;

    Ok(format!(
        "{:02}-{:02}-{:04} {:02}:{:02}:{:02}",
        day, month, year, hour, minute, second
    ))
}

/// Converts UNIX timestamp to (year, month, day)
fn unix_timestamp_to_date(timestamp: u64) -> (i32, u32, u32) {
    let days = timestamp / 86_400;
    let mut year = 1970;
    let mut remaining_days = days as i64;

    loop {
        let year_days = if is_leap_year(year) { 366 } else { 365 };
        if remaining_days >= year_days {
            remaining_days -= year_days;
            year += 1;
        } else {
            break;
        }
    }

    let month_days = if is_leap_year(year) {
        [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    } else {
        [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    };

    let mut month = 1;
    for &days_in_month in &month_days {
        if remaining_days >= days_in_month {
            remaining_days -= days_in_month;
            month += 1;
        } else {
            break;
        }
    }

    let day = remaining_days + 1;
    (year, month, day as u32)
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}
