use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct CustomStructType{
    pub jkc_u8: u8,
    
}

// 其他类型list同下面操作
#[derive(Debug, Serialize, Deserialize)]
pub struct VecExample{
    pub jkc_vec_u8: Vec<u8>,
    pub jkc_vec_custom_struct_type: Vec<CustomStructType>,
}


#[test]
fn test_vec_example() {
    let value = VecExample {
        jkc_vec_u8: vec![1, 2, 3, 4, 5],
        jkc_vec_custom_struct_type: vec![
            CustomStructType{jkc_u8: 1},
            CustomStructType{jkc_u8: 2},
        ],
    };

    let result = serde_json::to_string(&value).unwrap();
    println!("{:?}", result);
    // "{\"jkc_vec_u8\":[1,2,3,4,5],\"jkc_vec_custom_struct_type\":[{\"jkc_u8\":1},{\"jkc_u8\":2}]}"

    let value = r#"{"jkc_vec_u8":[1,2,3,4,5],"jkc_vec_custom_struct_type":[{"jkc_u8":1},{"jkc_u8":1}]}"#;
    let result: VecExample = serde_json::from_str(&value).unwrap();
    println!("{:?}", result);
    // VecExample { jkc_vec_u8: [1, 2, 3, 4, 5], jkc_vec_custom_struct_type: [CustomStructType { jkc_u8: 1 }, CustomStructType { jkc_u8: 1 }] }
}