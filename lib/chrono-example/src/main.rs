use std::str::FromStr;

use chrono::prelude::*;


fn main() {
    let timestamp: i64 = 132524746251044830;
    let datetime = NaiveDateTime::from_timestamp_micros (timestamp);
    println!("{:?}", datetime);
    // let datetime = NaiveDateTime::from_timestamp_opt(1701831914, 0);

    // 时间戳转时间
    println!("{:?}", NaiveDateTime::from_timestamp_opt(1701831914, 0)); // 2023-12-06T03:05:14
    println!("{:?}", NaiveDateTime::from_timestamp_opt(0, 1325247462)); // 2023-12-06T03:05:14

    if let Some(datetime) = DateTime::from_timestamp(1701831914, 0) {
        println!("{:?}", datetime.to_rfc2822()); // "Wed, 6 Dec 2023 03:05:14 +0000"
        println!("{:?}", datetime.to_rfc3339()); // "2023-12-06T03:05:14+00:00"
        println!("{:?}", datetime.to_rfc3339_opts(SecondsFormat::Millis, false)); // "2023-12-06T03:05:14.000+00:00"

        let datetime_local: DateTime<Local> = datetime.into();

        println!("{:?}", datetime_local); // 2023-12-06T11:05:14+08:00
    }

    let datetime = "2022-08-25T14:30:48+08:00".parse::<DateTime<Local>>();
    println!("{datetime:?}");

    let datetime = DateTime::parse_from_str("2014-11-28 21:00:09 +09:00", "%Y-%m-%d %H:%M:%S %z");
    println!("{datetime:?}");

    let datetime = NaiveDateTime::parse_from_str("2014-11-28 21:00:09", "%Y-%m-%d %H:%M:%S");
    println!("{datetime:?}");
}
