use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub enum EnumExample{
    Read,
    Write,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct StructExample {
    pub jkc_enum: EnumExample,
}


#[test]
fn test_enum_example() {
    let value = StructExample {
        jkc_enum: EnumExample::Read,
    };

    let result = serde_json::to_string(&value).unwrap();
    println!("{:?}", result);
    // "{\"jkc_enum\":\"Read\"}"

    let value = result;
    let result: StructExample = serde_json::from_str(&value).unwrap();
    println!("{:?}", result);
    // StructExample { jkc_enum: Read }
}
