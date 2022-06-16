use std::collections::HashMap;
use std::sync::RwLock;
use lazy_static::lazy_static;
use ctor::*;

// [dependencies]
// lazy_static = "*"
// ctor = "*"

type JkcCallback<'a> = fn(a: &'a [u8]);

lazy_static! {
    static ref MAP: RwLock<HashMap<u16, JkcCallback<'static>>> = {
        let map = HashMap::new();
        RwLock::new(map)
    };
}


fn jkc_callback<'a>(a: &'a [u8]) {
    println!("callback2: {:?}", a);
}


#[ctor]
fn init() {
    // 初始化函数, 在main函数执行之前执行
    let mut map = MAP.write().unwrap();

    map.insert(1, jkc_callback);
}


#[test]
fn test_callback() {
    let value = &[0x00, 0x01, 0x02, 0x03, 0x04];
    let map = MAP.read().unwrap();

    for (_id, func) in map.iter() {
        func(value);
    }
}
