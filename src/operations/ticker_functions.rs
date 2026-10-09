// here we get the right ticker for the
use chrono::Utc;
use chrono::{Datelike, Timelike};
use chrono_tz::America::New_York;

pub mod functions {

    use super::*;
    
    // returns the current active ticker for a updown market
    pub fn get_up_down_ticker(series: &str) -> String {

        let today: chrono::prelude::DateTime<chrono_tz::Tz> = Utc::now().with_timezone(&New_York);

        let month: &str = get_month(&today);
        let minute: u32 = today.minute();
        let mut hour: u32 = today.hour();
        let date = if today.day() > 10 {
            format!("{}", today.day())
        } else {
            format!("0{}", today.day())
        };

        let ticker_name: String = if minute < 15 {
            let ticker: String = if hour < 10 {
                format!(
                    "{}-{}{}{}0{}{}-{}",
                    series,
                    today.year() - 2000,
                    month,
                    date,
                    hour,
                    15,
                    15
                )
            } else {
                format!(
                    "{}-{}{}{}{}{}-{}",
                    series,
                    today.year() - 2000,
                    month,
                    date,
                    hour,
                    15,
                    15
                )
            };

            ticker
        } else if minute >= 15 && minute < 30 {
            let ticker: String = if hour < 10 {
                format!(
                    "{}-{}{}{}0{}{}-{}",
                    series,
                    today.year() - 2000,
                    month,
                    date,
                    hour,
                    30,
                    30
                )
            } else {
                format!(
                    "{}-{}{}{}{}{}-{}",
                    series,
                    today.year() - 2000,
                    month,
                    date,
                    hour,
                    30,
                    30
                )
            };

            ticker
        } else if minute >= 30 && minute < 45 {
            let ticker: String = if hour < 10 {
                format!(
                    "{}-{}{}{}0{}{}-{}",
                    series,
                    today.year() - 2000,
                    month,
                    date,
                    hour,
                    45,
                    45
                )
            } else {
                format!(
                    "{}-{}{}{}{}{}-{}",
                    series,
                    today.year() - 2000,
                    month,
                    date,
                    hour,
                    45,
                    45
                )
            };

            ticker
        } else if minute >= 45 && minute < 60 {
            hour += 1;
            let ticker: String = if hour < 10 {
                format!(
                    "{}-{}{}{}0{}{}-{}",
                    series,
                    today.year() - 2000,
                    month,
                    date,
                    hour,
                    "00",
                    "00"
                )
            } else {
                format!(
                    "{}-{}{}{}{}{}-{}",
                    series,
                    today.year() - 2000,
                    month,
                    date,
                    hour,
                    "00",
                    "00"
                )
            };

            ticker
        } else {
            panic!("something went wrong in getting ticker name")
        };

        ticker_name
    }

    // gets the month in string format
    fn get_month(date: &chrono::prelude::DateTime<chrono_tz::Tz>) -> &str {
        match date.month() {
            1 => "JAN",
            2 => "FEB",
            3 => "MAR",
            4 => "APR",
            5 => "MAY",
            6 => "JUN",
            7 => "JUL",
            8 => "AUG",
            9 => "SEP",
            10 => "OCT",
            11 => "NOV",
            12 => "DEC",
            _ => panic!("somehow the month was not one of the 12"),
        }
    }
}

