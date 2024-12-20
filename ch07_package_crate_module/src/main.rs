fn main() {
    // 模块系统的第一部分，我们将介绍包和 crate。crate 是一个二进制项或者库。crate root 是一个源文件，
    // Rust 编译器以它为起始点，并构成你的 crate 的根模块（我们将在“定义模块来控制作用域与私有性”一节深入解读）。
    // 包（package） 是提供一系列功能的一个或者多个 crate。一个包会包含有一个 Cargo.toml 文件，阐述如何去构建这些 crate。
    //
    // 包中所包含的内容由几条规则来确立。一个包中至多 只能 包含一个 库 crate(library crate)；
    // 包中可以包含任意多个 二进制 crate(binary crate)；包中至少包含一个 crate，无论是库的还是二进制的。

    // 在此，我们有了一个只包含 src/main.rs 的包，意味着它只含有一个名为 ch07_package_crate_module 的二进制 crate。
    // 如果一个包同时含有 src/main.rs 和 src/lib.rs，则它有两个 crate：一个库和一个二进制项，且名字都与包相同。
    // 通过将文件放在 src/bin 目录下，一个包可以拥有多个二进制 crate：每个 src/bin 下的文件都会被编译成一个独立的二进制 crate。
    println!("Hello, world!");
}
