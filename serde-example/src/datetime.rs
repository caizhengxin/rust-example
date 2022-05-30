#[allow(unused_imports)]
use chrono::prelude::*;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
struct JkcDateTime {
    #[serde(with = "datetime_format")]
    datetimestart: DateTime<Local>,
    #[serde(with = "datetime_format")]
    datetimestop: DateTime<Local>, 
}
 

mod datetime_format { 
    use chrono::prelude::*;
    use serde::{self, Deserializer, Serializer};
    use serde::de::Visitor;
 
    pub fn serialize<S>(datetime: &DateTime<Local>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&datetime.format("%Y-%m-%d %H:%M:%S").to_string())
    }
 
 
    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Local>, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(DateTimeVisitor)
    }
 
    struct DateTimeVisitor;
 
    impl<'de> Visitor<'de> for DateTimeVisitor {
        type Value = DateTime<Local>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("deserialize datetime error.")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error
        {
            let result = NaiveDateTime::parse_from_str(v, "%Y-%m-%d %H:%M:%S");

            if result.is_err() {
                return Err(E::custom(format!("deserialize datetime error {}.", v)));
            }            

            Ok(Local.from_local_datetime(&result.unwrap()).unwrap())
        } 
    }
}


#[test]
fn test_datetime() { 
    let jsonstr = r#"
        {
            "datetimestart": "2022-05-30 19:00:00",
            "datetimestop": "2022-05-30 19:00:00"
        }
    "#;
 
    let data: JkcDateTime = serde_json::from_str(jsonstr).unwrap();
    println!("{:#?}", data);

    // JkcDateTime {
    //     datetimestart: 2022-05-30T19:00:00+08:00,
    //     datetimestop: 2022-05-30T19:00:00+08:00,
    // }
 
    let serialized = serde_json::to_string_pretty(&data).unwrap();
    println!("{}", serialized);

    // {
    //     "datetimestart": "2022-05-30 19:00:00",
    //     "datetimestop": "2022-05-30 19:00:00"
    // }  
}
