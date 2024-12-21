fn main() {
    // 现在的目录结构
    // ├── Cargo.lock
    // ├── Cargo.toml
    // ├── add_one
    // │   ├── Cargo.toml
    // │   └── src
    // │       └── lib.rs
    // ├── adder
    // │   ├── Cargo.toml
    // │   └── src
    // │       └── main.rs
    // └── target
    let num = 16;

    println!("Hello, world! {} plus one is {}!", num, add_one::add_one(num));

    // 更多详情查看
    // https://www.rustwiki.org.cn/zh-CN/book/ch14-00-more-about-cargo.html
}
