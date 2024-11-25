// Title: normalize
// Created by sorryu
// Date: 2024-11-22
// Description: normalize data

/*
History(ex: 20xx-xx-xx | Modifications(what, how, why) | name)
2024-11-22 | 

*/

use regex::Regex;
use chrono::{ NaiveDate, Datelike, Weekday };



pub fn norm_email(email: &str) -> Result<String, String> {
    let email = email.trim();
    let email_regex = Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]+$").unwrap();

    if email_regex.is_match(&email) {
        Ok(email.to_string())
    } else {
        Err("Invalid email format".to_string())
    }
}

pub fn norm_phone_number(number: &str) -> Result<String, String> {
    let number: String = number.chars().filter(|c| c.is_digit(10)).collect();

    if number.len() >= 10 {
        Ok(number)
    } else {
        Err("Invalid phone number".to_string())
    }
}

pub fn norm_date(input_date: &str) -> Result<String, String> {
    let date = NaiveDate::parse_from_str(input_date, "%Y-%m-%d")
        .map_err(|_| "Invalid date format. Please use YYYY-MM-DD.".to_string())?;

    let weekday = date.weekday();

    let weekday_str = match weekday {
        Weekday::Mon => "MON",
        Weekday::Tue => "TUE",
        Weekday::Wed => "WED",
        Weekday::Thu => "THU",
        Weekday::Fri => "FRI",
        Weekday::Sat => "SAT",
        Weekday::Sun => "SUN",
    };

    Ok(format!("{}-{:02}-{:02}({})", date.year(), date.month(), date.day(), weekday_str))
}