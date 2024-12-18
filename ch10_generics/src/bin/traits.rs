use std::fmt::{Debug, Display};

pub trait Summary {
    fn summarize(&self) -> String;
    // 也可以有默认实现
    // 重载的话就使用新的 不重载就使用默认的
    // fn summarize(&self) -> String{
    //     "(Read more...)".to_string()
    // }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

impl Summary for Vec<String> {
    fn summarize(&self) -> String {
        let mut s = String::new();
        for x in self.iter() {
            s.push_str(x);
        }
        s
    }
}

fn main() {
    // trait 告诉 Rust 编译器某个特定类型拥有可能与其他类型共享的功能。可以通过 trait 以一种抽象的方式定义共享的行为。
    // 可以使用 trait bounds 指定泛型是任何拥有特定行为的类型。
    // 注意：trait 类似于其他语言中常被称为 接口（interfaces）的功能，虽然有一些不同。

    // 1. trait的一些基础使用
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let str_vec = vec![String::from("Hello, "), String::from("world")];
    println!("{}", str_vec.summarize());

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest2(&char_list);
    println!("The largest char is {}", result);
}


// 2. 下面是一些trait 作为参数的例子
pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Trait Bound 语法
pub fn notify2<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}

// 使用+号可以传入多个trait
pub fn notify3(item: impl Summary + Display) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify4<T: Summary + Display>(item: T) {
    println!("Breaking news! {}", item.summarize());
}

// 通过 where 简化 trait bound
fn some_function<T, U>(t: T, u: U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    1
}


// 3. 返回trait
fn return_summary() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

// 4. 实现largest函数
// 在 largest 函数体中我们想要使用大于运算符（>）比较两个 T 类型的值。这个运算符被定义为标准库中 trait std::cmp::PartialOrd 的一个默认方法。
// 所以需要在 T 的 trait bound 中指定 PartialOrd，这样 largest 函数可以用于任何可以比较大小的类型的 slice。
// 因为 PartialOrd 位于 prelude 中所以并不需要手动将其引入作用域。
//
// 对于非泛型版本的 largest 函数，我们只尝试了寻找最大的 i32 和 char。正如第 4 章 “只在栈上的数据：拷贝” 部分讨论过的
// ，像 i32 和 char 这样的类型是已知大小的并可以储存在栈上，所以他们实现了 Copy trait。
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// 返回引用的话就不用实现Copy trait了
fn largest2<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// 5. 使用 trait bound 有条件地实现方法
// 通过使用带有 trait bound 的泛型参数的 impl 块，可以有条件地只为那些实现了特定 trait 的类型实现方法。例如，示例
// 中的类型 Pair<T> 总是实现了 new 方法，不过只有那些为 T 类型实现了 PartialOrd trait （来允许比较） 和
// Display trait （来启用打印）的 Pair<T> 才会实现 cmp_display
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

