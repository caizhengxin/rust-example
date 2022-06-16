#[allow(unused_imports)]
use std::process::Command;


#[test]
fn test_shell_example() {
    // let output = Command::new("ls").output().expect("执行异常");
    // let output = Command::new("sh").arg("-c").arg(" -l -a").output().expect("命令执行异常错误提示");

    let output = Command::new("supervisorctl").args(["status"]).output().expect("执行异常");

    // println!("{:?}", output);

    let svlist = std::str::from_utf8(&output.stdout).unwrap();
    let text: Vec<&str> = svlist.split("\n").collect();

    for v in text {
        println!("{:?}", v);
    }
}