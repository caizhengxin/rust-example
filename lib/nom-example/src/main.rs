use nom::sequence::delimited;
use nom::bytes::complete::tag;
use nom::Err;
use nom::error::ErrorKind;


fn main() {
    let mut parser = delimited(tag("("), tag("abc"), tag(")"));

    assert_eq!(parser("sss(abc)"), Ok(("", "abc")));
    assert_eq!(parser("(abc)def"), Ok(("def", "abc")));
    assert_eq!(parser(""), Err(Err::Error(("", ErrorKind::Tag))));
    assert_eq!(parser("123"), Err(Err::Error(("123", ErrorKind::Tag))));
}
