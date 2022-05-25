use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum EnumExample{
    Read,
    Write,
}


#[test]
fn test_enum_example() {
    let value = EnumExample::Read;

    let result = serde_json::to_string(&value).unwrap();
    println!("{:?}", result);
    // "{\"type\":\"Read\"}"

    let value = result;
    let result: EnumExample = serde_json::from_str(&value).unwrap();
    println!("{:?}", result);
    // Read
}


#[derive(Debug, Serialize, Deserialize)]
pub struct StructExample {
    pub jkc_enum: EnumExample,
}


#[test]
fn test_enum_example2() {
    let value = StructExample {
        jkc_enum: EnumExample::Read,
    };

    let result = serde_json::to_string(&value).unwrap();
    println!("{:?}", result);
    // "{\"jkc_enum\":{\"type\":\"Read\"}}"

    let value = result;
    let result: StructExample = serde_json::from_str(&value).unwrap();
    println!("{:?}", result);
    // StructExample { jkc_enum: Read }
}


// 根据type数字类型，映射对应枚举
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum EnumExample2{
    #[serde(rename = "1", alias = "0x01")]
    Read {
        address: Option<u16>,
    },
    #[serde(rename = "2", alias = "0x02")]
    Write {
        address: Option<u16>,
        value: Option<u16>,
    },
    #[serde(other)]
    Other,
}


#[test]
fn test_enum_example3() {
    let value = EnumExample2::Read{address: Some(18)};
    let result = serde_json::to_string(&value).unwrap();
    println!("{:?}", result);
    // "{\"type\":\"1\",\"address\":18}"

    let value = r#"{"type":"1","address":16}"#;
    let result: EnumExample2 = serde_json::from_str(&value).unwrap();
    println!("{:?}", result);
    // Read { address: Some(16) }

    let value = r#"{"type":"2","address":16,"value":19}"#;
    let result: EnumExample2 = serde_json::from_str(&value).unwrap();
    println!("{:?}", result);
    // Write { address: Some(16), value: Some(19) }

    let value = r#"{"type":"3"}"#;
    let result: EnumExample2 = serde_json::from_str(&value).unwrap();
    println!("{:?}", result);
    // Other
}
