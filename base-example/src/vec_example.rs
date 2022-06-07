#[test]
fn test_vec_join() {
    // 方法一, 该方法不支持双引号字符串
    let vec_str = vec!['j', 'k', 'c'];
    println!("Vec join: {}", vec_str.iter().collect::<String>());

    // 方法二, 该方法不支持单引号字符
    let vec_str = vec!["jan", "kin", "cai"];
    println!("Vec join: {}", vec_str.concat());

    // 方法三, 该方法不支持单引号字符
    let vec_str = vec!["jan", "kin", "cai"];
    println!("Vec join: {}", vec_str.join(""));
}
