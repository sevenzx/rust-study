use std::fmt::Result;
// 使用 as 指定一个新的本地名称或者别名
use std::io::Result as IoResult;

// 嵌套路径
// use std::cmp::Ordering;
// use std::io;
use std::{io, cmp::Ordering};

// 我们用关键字 mod 定义一个模块
// 在 mod front_of_house 后使用分号，而不是代码块，这将告诉 Rust 在另一个与模块同名的文件中加载模块的内容。
// 下面就是在加载front_of_house.rs
mod front_of_house;


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

// 我们还可以使用 pub 来设计公有的结构体和枚举，不过有一些额外的细节需要注意。
// 如果我们在一个结构体定义的前面使用了 pub ，这个结构体会变成公有的，但是这个结构体的字段仍然是私有的。
// 我们可以根据情况决定每个字段是否公有。

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // 如果我们将枚举设为公有，则它的所有成员都将变为公有。我们只需要在 enum 关键字前面加上 pub
    #[derive(Debug)]
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant2() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
    println!("{:?} {:?}", order1, order2)
}

fn function1() -> Result {
    // --snip--
    Result {}
}

fn function2() -> IoResult<()> {
    // --snip--
    IoResult {}
}