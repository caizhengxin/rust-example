use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct CustomStructType{
    pub jkc_u8: u8,
    
}

// 其他类型list同下面操作
#[derive(Debug, Serialize, Deserialize)]
pub struct ArrayExample{
    pub jkc_array_u8: [u8;6],
    pub jkc_array_custom_struct_type: [CustomStructType;2],
}


#[test]
fn test_array_example() {
    let value = ArrayExample {
        jkc_array_u8: [1, 2, 3, 4, 5, 6],           // 必须是6位数组，否则报错
        jkc_array_custom_struct_type: [             // 必须是2位数组，否则报错
            CustomStructType{jkc_u8: 1},
            CustomStructType{jkc_u8: 2},
        ],
    };

    let result = serde_json::to_string(&value).unwrap();
    println!("{:?}", result);
    // "{\"jkc_array_u8\":[1,2,3,4,5,6],\"jkc_array_custom_struct_type\":[{\"jkc_u8\":1},{\"jkc_u8\":2}]}"

    let value = r#"{"jkc_array_u8":[1,2,3,4,5,6],"jkc_array_custom_struct_type":[{"jkc_u8":1},{"jkc_u8":2}]}"#;
    let result: ArrayExample = serde_json::from_str(&value).unwrap();
    println!("{:?}", result);
    // ArrayExample { jkc_array_u8: [1, 2, 3, 4, 5, 6], jkc_array_custom_struct_type: [CustomStructType { jkc_u8: 1 }, CustomStructType { jkc_u8: 2 }] }
}
