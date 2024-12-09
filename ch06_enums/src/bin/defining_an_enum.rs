// 普通枚举定义
enum IpAddrKind {
    V4,
    V6,
}
// 1. 和其他语言枚举不同的是, Rust中可以将数据附加到枚举的每个成员上
// 这个枚举有四个含有不同类型的成员：
//
// Quit 没有关联任何数据。
// Move 包含一个匿名结构体。
// Write 包含单独一个 String。
// ChangeColor 包含三个 i32。
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}


// 2. 枚举和结构体还有另一个相似点：就像可以使用 impl 来为结构体定义方法那样，
// 也可以在枚举上定义方法。这是我们在 Message 枚举上定义了一个叫做 call 的方法
impl Message {
    fn call(&self) {
        println!("{:?}", &self);
    }
}

fn main() {
    let m = Message::Move { x: 1, y: 2 };
    m.call();

    // 3. Option<T> 有俩成员 Some None
    // 详情查看: https://www.rustwiki.org.cn/zh-CN/book/ch06-01-defining-an-enum.html#option-%E6%9E%9A%E4%B8%BE%E5%92%8C%E5%85%B6%E7%9B%B8%E5%AF%B9%E4%BA%8E%E7%A9%BA%E5%80%BC%E7%9A%84%E4%BC%98%E5%8A%BF
    // Option<T> 文档: https://www.rustwiki.org.cn/zh-CN/std/option/enum.Option.html
    // let x = Option::Some(5); 可以直接使用Some None
    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;


    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y.unwrap_or_default();
    println!("sum = {}", sum);
}