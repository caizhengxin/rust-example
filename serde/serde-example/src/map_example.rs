use std::collections::HashMap;
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct MapExample{
    pub jkc_map: HashMap<u16, String>,
}

#[test]
fn test_map_example() {
    let value = MapExample {
        jkc_map: HashMap::from([
            (1, "jkc1".to_string()),
            (2, "jkc2".to_string()),
            (3, "jkc3".to_string()),
        ]),
    };

    let result = serde_json::to_string(&value).unwrap();
    println!("{:?}", result);
    // "{\"jkc_map\":{\"1\":\"jkc1\",\"2\":\"jkc2\",\"3\":\"jkc3\"}}"

    let value = result;
    let result: MapExample = serde_json::from_str(&value).unwrap();
    println!("{:?}", result);
    // MapExample { jkc_map: {3: "jkc3", 1: "jkc1", 2: "jkc2"} }
}