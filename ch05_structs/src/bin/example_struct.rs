// Rust 确实包含了打印出调试信息的功能，不过我们必须为结构体显式选择这个功能。
// 为此，在结构体定义之前加上外部属性 #[derive(Debug)]
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}


fn main() {
    let x = 2;
    let rect = Rectangle {
        // dbg! 宏接收一个表达式的所有权，打印出代码中调用 dbg! 宏时
        // 所在的文件和行号，以及该表达式的结果值，并返回该值的所有权
        width: dbg!(x * 30), // [src/bin/example_struct.rs:15:16] x * 30 = 60
        height: 50,
    };
    println!("{:?}", rect);
    println!("{:#?}", rect);
    dbg!(&rect);
    let area = area(&rect);
    println!("area = {}", area); // 3000
}

fn area(rect: &Rectangle) -> u32 {
    rect.height * rect.width
}