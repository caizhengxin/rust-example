use std::collections::HashMap;
use std::sync::RwLock;
use lazy_static::lazy_static;
use ctor::*;

// [dependencies]
// lazy_static = "*"
// ctor = "*"

type JkcCallback = fn(a: u16, b: u16) -> u16;

lazy_static! {
    static ref MAP: RwLock<HashMap<u16, JkcCallback>> = {
        let map = HashMap::new();
        RwLock::new(map)
    };
}


fn jkc_callback(a: u16, b: u16) -> u16 {
    a + b
}


fn jkc_callback2(a: u16, b: u16) -> u16 {
    a - b
}


#[ctor]
fn init() {
    // 初始化函数, 在main函数执行之前执行
    let mut map = MAP.write().unwrap();

    map.insert(1, jkc_callback);
    map.insert(2, jkc_callback2);
}


#[test]
fn test_callback() {
    let map = MAP.read().unwrap();

    for (id, func) in map.iter() {
        println!("id={} result={}", id, func(10, 5));
    }

    let map = MAP.read().unwrap();

    let func = map.get(&1).unwrap();
    assert_eq!(func(10, 5), 15)
}
