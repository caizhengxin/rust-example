use std::collections::HashMap;
use std::sync::RwLock;
use lazy_static::lazy_static;

type JkcCallback<'a> = fn(a: &'a [u8]);

fn jkc_callback<'a>(a: &'a [u8]) {
    println!("static callback: {:?}", a);
}


lazy_static! {
    static ref FUNC_MAP: RwLock<StructExample<'static>> = {
        let mut head = StructExample::new();

        head.map.insert(1, jkc_callback);

        RwLock::new(head)
    };
}


struct StructExample<'a> {
    pub map: HashMap<u16, JkcCallback<'a>>,
}


impl<'a> StructExample<'a> {
    pub fn new() -> Self {
        StructExample {map: HashMap::new() }
    }

    pub fn is_callback(&self, _input: &'a [u8]) -> &JkcCallback<'a>
    {
        self.map.get(&1).unwrap()
    }
}


#[test]
fn test_callback() {
    let value = &[0x00, 0x01, 0x02, 0x03, 0x04];
    let head = FUNC_MAP.read().unwrap();

    let func = head.is_callback(&value[..]);

    func(value);
}
