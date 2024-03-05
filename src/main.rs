

use std::io;

fn main() {
    // 你的程序代码

    // 在这里添加等待输入的语句
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
}