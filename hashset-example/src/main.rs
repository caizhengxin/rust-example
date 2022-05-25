#![feature(hash_set_entry)]
use std::collections::HashSet;


fn main() {
    let mut head = HashSet::new();

    // 插入
    head.insert("jkc1".to_string());
    head.insert("jkc2".to_string());
    // 重复会被覆盖
    head.insert("jkc2".to_string());

    // 获取hash数量
    println!("[DEBUG]: head length {:?}", head.len());

    // 查询
    if head.contains("jkc1") {
        println!("[DEBUG]: contains jkc1 success!");
    }

    // 删除
    head.remove("jkc1");

    if !head.contains("jkc1") {
        println!("[DEBUG]: contains jkc1 fail!");
    }

    // 获取
    println!("{:?}", head.get("jkc2"));
    println!("{:?}", head.get_or_insert("jkc3".to_string()));


    // 遍历
    for v in &head {
        println!("{}", v);
    }
}
