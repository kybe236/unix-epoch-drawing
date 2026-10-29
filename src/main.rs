// Inspired by Sigmanificient profile.

use std::process::Command;

use chrono::{Datelike, Duration, NaiveDate, NaiveTime, Weekday};

fn main() -> anyhow::Result<()> {
    let start_date = NaiveDate::from_ymd_opt(1970, 1, 1).unwrap();
    let end_date = NaiveDate::from_ymd_opt(2020, 12, 31).unwrap();
    let time = NaiveTime::from_hms_opt(9, 6, 23).unwrap();

    let mut current_date = start_date;

    while current_date <= end_date {
        if current_date.weekday() == Weekday::Sat || current_date.weekday() == Weekday::Sun {
            let naive_dt = current_date.and_time(time);
            let git_date_str = naive_dt
                .and_utc()
                .format("%Y-%m-%d %H:%M:%S %z")
                .to_string();

            let output = Command::new("git")
                .env("GIT_AUTHOR_DATE", &git_date_str)
                .env("GIT_COMMITTER_DATE", &git_date_str)
                .args(["commit", "-m", "\u{200B}", "--allow-empty"])
                .output()?;

            let stdout = String::from_utf8(output.stdout)?;
            let stderr = String::from_utf8(output.stderr)?;
            eprintln!("n: {naive_dt} g: {git_date_str} o: {stdout} e: {stderr}")
        }

        current_date += Duration::days(1);
    }

    Ok(())
}
