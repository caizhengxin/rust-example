use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct Read {
    pub address: u16,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Write {
    pub address: u16,
    pub value: u16,
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EnumExample{
    Read(Read),
    Write(Write),
}


#[test]
fn test_enum_example() {
    let value = EnumExample::Read(Read{address: 12});

    let result = serde_json::to_string(&value).unwrap();
    println!(">>> {:?}", result);
    // "{\"address\":12}"
    // 如果不适用untagged, 结果{“Read”: {"address": 12}}

    let value = result;
    let result: EnumExample = serde_json::from_str(&value).unwrap();
    println!("{:?}", result);
    // Read(Read { address: 12 })
}
