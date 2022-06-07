#[allow(unused_imports)]
use std::fs::{self, File};
#[allow(unused_imports)]
use std::io::prelude::*;
#[test]
fn test_file_read() -> std::io::Result<()> {
    // 覆盖写入
    let mut file = File::create("jkc.txt")?;
    file.write(b"jankincai!\n")?;

    // 增量写入
    let mut file = File::options().append(true).open("jkc.txt")?;
    file.write(b"jankincai-12!\n")?;

    // Read
    let mut file = File::open("jkc.txt")?;
    let mut text = String::new();

    file.read_to_string(&mut text)?;
    println!("File Read: {:?}", text);

    // 读取字节流，类似[106, 97, ..., 100]
    let mut file = File::open("jkc.txt")?;
    let mut text = vec![];

    file.read_to_end(&mut text)?;
    println!("File Read: {:?}", text);

    fs::remove_file("jkc.txt")?;

    Ok(())
}
