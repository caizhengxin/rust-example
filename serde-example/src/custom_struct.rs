use std::fmt;
use serde::{Serialize, Deserialize};
use serde::de::{Deserializer, Visitor};


// 该例子主要学习如果对Struct类型，自定义序列化和反序列化内容


#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct JkcStruct1 {
    pub a: Option<u8>,
    pub b: Option<u8>,
}


#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct JkcStruct2 {
    pub a: Option<u8>,
    pub b: Option<u8>,
    pub c: Option<u8>,
    pub d: Option<u8>,
}


#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
// #[serde(untagged)]
// #[serde(tag = "t", content = "c")]
pub enum JkcEnum {
    JkcStruct1(JkcStruct1),
    JkcStruct2(JkcStruct2),
}


#[derive(Debug)]
pub struct JkcStruct {
    // #[serde(rename = "type")]
    pub tpe: u8,
    pub value: Option<Vec<JkcEnum>>,
}


impl Serialize for JkcStruct {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer
    {
        let mut state = serializer.serialize_struct("JkcStruct", 3)?;
        
        state.serialize_field("type", &self.tpe)?;
        state.serialize_field("value", &self.value)?;

        state.end()
    }
}


impl<'de> Deserialize<'de> for JkcStruct {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>
    {
        struct JkcStructVisitor;

        impl<'de> Visitor<'de> for JkcStructVisitor {
            type Value = JkcStruct;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("JkcStruct parse error.")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
                where
                    A: serde::de::MapAccess<'de>
            {
                let mut tpe: u8 = 0;
                let mut value: Option<serde_json::Value> = None;

                while let Some(key) = map.next_key::<&str>()? {
                    match key {
                        "type" | "tpe" => {
                            tpe = map.next_value()?;
                        },
                        "value" => {
                            value = map.next_value()?;
                        },
                        _ => {},
                    }
                }

                if value.is_none() {
                    return Ok(JkcStruct{tpe: tpe, value: None});
                }

                let mut value2: Option<Vec<JkcEnum>> = None;

                match tpe {
                    1 => {
                        let ret = serde_json::from_value::<Vec<JkcStruct1>>(value.unwrap());

                        match ret {
                            Ok(v) => {
                                value2 = Some(v.iter().map(|&v| JkcEnum::JkcStruct1(v)).collect());
                            },
                            Err(e) => {
                                return Err(serde::de::Error::custom(format!("deserialize JkcStruct error {}.", e)));
                            },
                        }
                    },
                    2 => {
                        let ret = serde_json::from_value::<Vec<JkcStruct2>>(value.unwrap());

                        match ret {
                            Ok(v) => {
                                value2 = Some(v.iter().map(|&v| JkcEnum::JkcStruct2(v)).collect());
                            },
                            Err(e) => {
                                return Err(serde::de::Error::custom(format!("deserialize JkcStruct error {}.", e)));
                            },
                        }
                    },
                    _ => {},
                }

                Ok(JkcStruct{tpe: tpe, value: value2})
            }
        }

        deserializer.deserialize_struct("", &[], JkcStructVisitor)
    }
}


#[test]
fn test_custom_struct() {
    let value = r#"{"type":2,"value":[{"a":1,"b":2}]}"#;

    let value: JkcStruct = serde_json::from_str(&value).unwrap();
    println!("{:?}", value);

    let value = serde_json::to_string(&value).unwrap();
    println!("{:?}", value);

    let value = r#"{"type":1,"value":[{"a":1,"b":2}]}"#;

    let value: JkcStruct = serde_json::from_str(&value).unwrap();
    println!("{:?}", value);

    let value = serde_json::to_string(&value).unwrap();
    println!("{:?}", value);
}
