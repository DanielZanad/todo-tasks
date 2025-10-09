use chrono::{DateTime, Datelike, Local, Timelike, Utc};
use chrono::{TimeZone, offset::LocalResult};
use sqlx::types::time::PrimitiveDateTime;
use sqlx::types::time::{Date, Time};

use actix_web::cookie::time::Month::{self, *};

pub fn to_domain(date: PrimitiveDateTime) -> LocalResult<DateTime<Utc>> {
    let month = match date.month() {
        January => 1,
        February => 2,
        March => 3,
        April => 4,
        May => 5,
        June => 6,
        July => 7,
        August => 8,
        September => 9,
        October => 10,
        November => 11,
        December => 12,
    };
    let new_date = Utc.with_ymd_and_hms(
        date.year(),
        month,
        date.day() as u32,
        date.hour() as u32,
        date.minute() as u32,
        date.second() as u32,
    );
    new_date
}

pub fn chrono_to_primitive(datetime: DateTime<Utc>) -> PrimitiveDateTime {
    let month = match datetime.month() {
        1 => Month::January,
        2 => Month::February,
        3 => Month::March,
        4 => Month::April,
        5 => Month::May,
        6 => Month::June,
        7 => Month::July,
        8 => Month::August,
        9 => Month::September,
        10 => Month::October,
        11 => Month::November,
        _ => Month::December,
    };

    let date = Date::from_calendar_date(datetime.year(), month, datetime.day() as u8).unwrap();

    let time = Time::from_hms_milli(
        datetime.hour() as u8,
        datetime.minute() as u8,
        datetime.second() as u8,
        datetime.timestamp_subsec_millis() as u16,
    )
    .unwrap();

    PrimitiveDateTime::new(date, time)
}
