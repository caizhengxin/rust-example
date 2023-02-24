use std::collections::HashMap;
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct MapExample{
    pub jkc_string: String,
    #[serde(flatten)]                           // 通过flatten，展开HashMap里面的内容
    pub jkc_map: HashMap<String, u16>,
}


#[test]
fn test_map_example() {
    let value = MapExample {
        jkc_string: "jankincai".to_string(),
        jkc_map: HashMap::from([
            ("jkc1".to_string(), 1),
            ("jkc2".to_string(), 2),
            ("jkc3".to_string(), 3),
        ]),
    };

    let result = serde_json::to_string(&value).unwrap();
    println!("Map flatten: {:?}", result);
    // "{\"jkc_string\":\"jankincai\",\"jkc3\":3,\"jkc1\":1,\"jkc2\":2}"

    let value = result;
    let result: MapExample = serde_json::from_str(&value).unwrap();
    println!("Map flatten: {:?}", result);
    // MapExample { jkc_string: "jankincai", jkc_map: {"jkc1": 1, "jkc3": 3, "jkc2": 2} }
}