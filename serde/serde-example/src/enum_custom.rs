use std::fmt;
use serde::{Serialize, Serializer, Deserialize, Deserializer};
use serde::de::{self, Visitor};


#[derive(Debug, Default, Clone, Copy)]
pub enum EnumCustomExample{
    #[default]
    Read = 1,
    Write,
}


impl EnumCustomExample {
    pub fn from(value: u8) -> Option<Self> {
        if value == EnumCustomExample::Read as u8 { return Some(EnumCustomExample::Read); }
        else if value == EnumCustomExample::Write as u8 { return Some(EnumCustomExample::Write); }
        else { return None; }
    }
}


impl Serialize for EnumCustomExample {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u8(*self as u8)
    }
}


impl<'de> Deserialize<'de> for EnumCustomExample {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        struct EnumCustomExampleVisitor;

        impl<'de> Visitor<'de> for EnumCustomExampleVisitor {
            type Value = EnumCustomExample;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("enum custom example error")
            }

            fn visit_u8<E>(self, value: u8) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if value == EnumCustomExample::Read as u8 { return Ok(EnumCustomExample::Read); }
                else if value == EnumCustomExample::Write as u8 { return Ok(EnumCustomExample::Write); }
                else { return Err(E::custom(format!("enum custom example error"))); }
            }

            fn visit_u16<E>(self, value: u16) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                self.visit_u8(value as u8)
            }

            fn visit_u32<E>(self, value: u32) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                self.visit_u8(value as u8)
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                self.visit_u8(value as u8)
            }
        }

        // 如果不实现所有的u8、u16、u32、u64，则会报错
        deserializer.deserialize_u8(EnumCustomExampleVisitor)
        // deserializer.deserialize_u16(EnumCustomExampleVisitor)
        // deserializer.deserialize_u32(EnumCustomExampleVisitor)
        // deserializer.deserialize_u64(EnumCustomExampleVisitor)
    }
}


#[derive(Debug, Serialize, Deserialize, Default)]
pub struct StructExample {
    pub jkc_enum: EnumCustomExample,
}


#[test]
fn test_enum_example() {
    let value = StructExample::default();

    let result = serde_json::to_string(&value).unwrap();
    println!("enum custom serialize: {:?}", result);
    // "{\"jkc_enum\":0}"

    let value = result;
    let result: StructExample = serde_json::from_str(&value).unwrap();
    println!("enum custom deserialize: {:?}", result);
    // StructExample { jkc_enum: Read }
}
