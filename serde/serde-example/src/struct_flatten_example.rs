use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct StructFlattenExample2{
    pub jkc_u8: u8,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct StructFlattenExample{
    pub jkc_string: String,
    #[serde(flatten)]                           // 通过flatten，展开HashMap里面的内容
    pub jkc_struct: StructFlattenExample2,
}


#[test]
fn test_struct_flatten() {
    let value = StructFlattenExample {
        jkc_string: "jankincai".to_string(),
        jkc_struct: StructFlattenExample2 {
            jkc_u8: 1,
        }
    };

    let result = serde_json::to_string(&value).unwrap();
    println!("Struct flatten: {:?}", result);
    // 不加flatten: "{\"jkc_string\":\"jankincai\",\"jkc_struct\":{\"jkc_u8\":1}}"
    // 加了flatten: "{\"jkc_string\":\"jankincai\",\"jkc_u8\":1}"

    let value = result;
    let result: StructFlattenExample = serde_json::from_str(&value).unwrap();
    println!("Struct flatten: {:?}", result);
    // StructFlattenExample { jkc_string: "jankincai", jkc_struct: StructFlattenExample2 { jkc_u8: 1 } }
}