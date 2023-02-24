use nom::bytes::complete::{tag, take_until, take_while};
use nom::character::complete::digit0;
use nom::combinator::peek;
use nom::sequence::tuple;
use nom::multi::{fill, many0};
use nom::character::complete::{alpha1, alpha0, space0, crlf};
use nom::number::complete::{u8};
use nom::character::is_digit;


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TelnetCmd {
    pub cmd: u8,
    pub subcmd: Option<u8>,
}


#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TelnetData<'a> {
    pub data: Vec<&'a [u8]>,
}


#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TelnetHeader<'a> {
    TelnetCmd(Vec<TelnetCmd>),
    TelnetData(TelnetData<'a>),
}


pub fn is_not_ff(chr: u8) -> bool {
    chr != 0xff
}  


pub fn get_subcmd(input: &[u8]) -> Option<u8> {
    if input.len() > 0 {
        return Some(input[0]);
    }

    None
}


pub fn parse_telnet_command<'a>(input: &'a [u8]) -> nom::IResult<&[u8], Vec<TelnetCmd>> {
    let (input, data) = many0(tuple((tag(b"\xff"), u8, take_while(is_not_ff), peek(many0(tag(b"\xff"))))))(input)?;

    let vlist = data.iter().map(|v| TelnetCmd { cmd: v.1, subcmd: get_subcmd(v.2) }).collect();

    Ok((input, vlist))
}


pub fn parse_telnet_data<'a>(input: &'a [u8]) -> nom::IResult<&[u8], Vec<&'a [u8]>> {
    let (input, data) = many0(tuple((take_until("\r\n"), crlf)))(input)?;

    let vlist = data.iter().map(|v| v.0).collect();

    Ok((input, vlist))
}


pub fn parse_telnet_header(input: &[u8]) -> nom::IResult<&[u8], TelnetHeader> {
    if input[0] == 0xff {
        // 命令解析
        let (input, vlist) = parse_telnet_command(input)?;

        return Ok((input, TelnetHeader::TelnetCmd(vlist)));
    }
    else {
        // 数据解析

        let (input, vlist) = parse_telnet_data(input)?;

        return Ok((input, TelnetHeader::TelnetData(TelnetData{ data: vlist})));
    }
}


#[test]
fn test_telnet() {
    let data = b"PING www.yahoo.com (204.71.200.67): 56 data bytes\r\njankincai\r\n";

    let value = parse_telnet_header(data);

    println!("{:?}", value);

    let data = &[
        0xff, 0xfd, 0x03, 0xff, 0xfb, 0x18, 0xff, 0xfb,
        0x1f, 0xff, 0xfb, 0x20, 0xff, 0xfb, 0x21, 0xff,
        0xfb, 0x22, 0xff, 0xfb, 0x27, 0xff, 0xfd, 0x05,
        0xff, 0xfa, 0x22, 0x01, 0x0b,
        0xff, 0xfb, 0x23,
    ];

    let data = &[
        0x24, 0x20
    ];

    let value = parse_telnet_header(data);

    println!("{:?}", value);
}
