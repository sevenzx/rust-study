// 我们用关键字 mod 定义一个模块
mod front_of_house {
    // 使用 pub 关键字公有暴露路径
    pub mod hosting {
        pub fn add_to_waitlist() {
            // Rust 中默认所有项（函数、方法、结构体、枚举、模块和常量）都是私有的。父模块中的项不能使用子模块中的私有项，
            // 但是子模块中的项可以使用他们父模块中的项。这是因为子模块封装并隐藏了他们的实现详情，
            // 但是子模块可以看到他们定义的上下文。

            // 1. 这里的serving不是pub的mod却可以使用 是因为可以使用父模块中的项
            // 2. 这里使用了super关键字 我们还可以使用 super 开头来构建从父模块开始的相对路径。类似于..
            super::serving::take_order();
        }
        fn seat_at_table() {}
    }
    mod serving {
        pub fn take_order() {}
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