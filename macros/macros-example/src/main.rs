// use std::string::ToString;
// use macros_example::{show_streams};
use macros_example::JkcDerive;


#[derive(Debug)]
pub enum PacketError {
    LengthError,
}


pub trait PacketDecode: Sized {
    fn decode(input: &[u8]) -> Result<(Self, &[u8]), PacketError>;
}


impl PacketDecode for u16 {
    fn decode(input: &[u8]) -> Result<(Self, &[u8]), PacketError> {
        Ok((
            1,
            input,
        ))
    }
}


#[derive(Debug, JkcDerive)]
// #[show_streams(byteorder=">")]
pub struct JkcStruct {
    pub a: u16,
    pub b: u8,
}


fn main() {
    let jkc = JkcStruct{a: 1, b: 2};

    println!{"{:?}", jkc};
    // jkc.jankincai();

    println!("{:?}", u16::decode(&[1,2,3,4]));
}
