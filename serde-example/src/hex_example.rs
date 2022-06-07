use serde::{Serialize, Deserialize};


// Cargo.toml: hex = { version = "0.4", features = ["serde"]}


#[derive(Debug, Serialize, Deserialize)]
pub struct HexExample {
    #[serde(with = "hex")]
    pub jkc_hex: Vec<u8>,
}


#[test]
fn test_struct_flatten() {
    let value = HexExample {
        jkc_hex: vec![1,2,3,4,5,6,7],
    };

    let result = serde_json::to_string(&value).unwrap();
    println!("Hex Example: {:?}", result);
    // "{\"jkc_hex\":\"01020304050607\"}"

    let value = result;
    let result: HexExample = serde_json::from_str(&value).unwrap();
    println!("Hex Example: {:?}", result);
    // HexExample { jkc_hex: [1, 2, 3, 4, 5, 6, 7] }
}
