use std::{env, fs};

fn main() {
    // Rust 的运行速度、安全性、单二进制文件输出和跨平台支持使其成为创建命令行程序的绝佳选择，
    // 所以我们的项目将创建一个我们自己版本的经典命令行工具：grep。
    // grep 是 “Globally search a Regular Expression and Print.” 的首字母缩写。
    // grep 最简单的使用场景是在特定文件中搜索指定字符串。为此，grep 获取一个文件名和一个字符串作为参数，
    // 接着读取文件并找到其中包含字符串参数的行，然后打印出这些行。

    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);

    let content = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    println!("With text:\n{}", content);
}
