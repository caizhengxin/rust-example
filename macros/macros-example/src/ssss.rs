
// char
// bool
// u8、u16、u32、u64、u128, 可能存在u24
// i8、i16、i32、i64、i128, 可能存在i24
// f32、f64
// string
// hex
// array
// IP、MAC
// UUID
// DateTime、UTC...

// #[packet(byteorder=">")]
// pub struct JkcStruct {
//     pub jkc_u16: u8,
//     #[packet(byteorder="<")]
//     pub jkc_u16: u16,
//     pub jkc_u32: u32,
//     pub jkc_u64: u64,
//     pub jkc_u128: u128,
//     pub jkc_string: String,
//     pub jkc_f32: f32,
//     pub jkc_f64: f64,
//     pub jkc_ip: IP,
// }
