use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct BaseExample{
    pub jkc_u8: u8,
    pub jkc_string: String,
    pub jkc_bool: bool,
    pub jkc_float: f32,
}


#[test]
fn test_base_example() {
    let value = BaseExample {
        jkc_u8: 1,
        jkc_string: "jankincai".to_string(),
        jkc_bool: true,
        jkc_float: 2.0,
    };

    let result = serde_json::to_string(&value).unwrap();
    println!("{:?}", result);
    // "{\"jkc_u8\":1,\"jkc_string\":\"jankincai\",\"jkc_bool\":true,\"jkc_float\":2.0}"

    let value = r#"{"jkc_u8":1,"jkc_string":"jankincai","jkc_bool":true,"jkc_float":2.0}"#;
    let result: BaseExample = serde_json::from_str(&value).unwrap();
    println!("{:?}", result);
}


// &str类型，需要标注生命周期
#[derive(Debug, Serialize, Deserialize)]
pub struct BaseExample2<'a>{
    pub jkc_str: &'a str,
}


#[test]
fn test_base_example2() {
    let value = BaseExample2 {
        jkc_str: "jankincai",
    };

    let result = serde_json::to_string(&value).unwrap();
    println!("{:?}", result);

    let value = r#"{"jkc_str":"jankincai"}"#;
    let result: BaseExample2 = serde_json::from_str(&value).unwrap();
    println!("{:?}", result);
}