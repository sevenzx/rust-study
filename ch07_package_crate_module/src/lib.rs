// 我们用关键字 mod 定义一个模块
mod front_of_house {
    // 使用 pub 关键字公有暴露路径
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }
    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}
pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

// 之前我们提到，src/main.rs 和 src/lib.rs 被称为 crate 根。如此称呼的原因是，这两个文件中任意一个的内容会构成名为 crate 的模块，
// 且该模块位于 crate 的被称为 模块树 的模块结构的根部（"at the root of the crate’s module structure"）。
// crate
//  └── front_of_house
//      ├── hosting
//      │   ├── add_to_waitlist
//      │   └── seat_at_table
//      └── serving
//          ├── take_order
//          ├── serve_order
//          └── take_payment