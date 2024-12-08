fn main() {
    // 1. 引用和借用
    reference();
    // 2. 可变引用
    mutable_reference();
    // 3. 悬垂引用
    // let s = dangle();
    let s = no_dangle();
    println!("s: {}", s);

    // 引用的规则
    // 让我们概括一下之前对引用的讨论：
    //
    // 在任意给定时间，要么 只能有一个可变引用，要么 只能有多个不可变引用。
    // 引用必须总是有效的。
}

fn reference() {
    let s1 = String::from("hello");
    // 我们将创建一个引用的行为称为 借用（borrowing）。
    // 正如现实生活中，如果一个人拥有某样东西，你可以从他那里借来。当你使用完毕，必须还回去。

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize { // s 是对 String 的引用
    // s -> s1 -> 堆上的"hello"
    // 注意, 这里的s是不能修改的。
    // s.push_str(", world!");
    s.len()
} // 这里，s 离开了作用域。但因为它并不拥有引用值的所有权，
// 所以什么也不会发生

fn mutable_reference() {
    let mut s = String::from("hello");
    change(&mut s);
    println!("s: {}.", s);

    // 1. 不过可变引用有一个很大的限制：在同一时间，只能有一个对某一特定数据的可变引用。
    // 尝试创建两个可变引用的代码将会失败
    // let r1 = &mut s;
    // let r2 = &mut s;
    // println!("{}, {}", r1, r2);
    // 这个限制的好处是 Rust 可以在编译时就避免数据竞争。数据竞争（data race）类似于竞态条件，它可由这三个行为造成：
    //
    // 两个或更多指针同时访问同一数据。
    // 至少有一个指针被用来写入数据。
    // 没有同步数据访问的机制。

    // 2. 一如既往，可以使用大括号来创建一个新的作用域，以允许拥有多个可变引用，只是不能 同时 拥有
    {
        let r1 = &mut s;
        println!("r1 {}", r1);
    } // r1 在这里离开了作用域，所以我们完全可以创建一个新的引用

    let r2 = &mut s;
    println!("r2 {}", r2);

    // 3. 类似的规则也存在于同时使用可变与不可变引用中。
    // let r1 = &s; // 没问题
    // let r2 = &s; // 没问题
    // let r3 = &mut s; // 大问题
    // println!("{}, {}, and {}", r1, r2, r3);
    // 我们 也 不能在拥有不可变引用的同时拥有可变引用。不可变引用的用户可不希望在他们的眼皮底下值就被意外的改变了！
    // 然而，多个不可变引用是可以的，因为没有哪个只能读取数据的人有能力影响其他人读取到的数据。

    // 4. 注意一个引用的作用域从声明的地方开始一直持续到最后一次使用为止。
    // 例如，因为最后一次使用不可变引用（println!)，发生在声明可变引用之前，所以如下代码是可以编译的：
    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
    println!("{} and {}", r1, r2);
    // 此位置之后 r1 和 r2 不再使用

    let r3 = &mut s; // 没问题
    println!("{}", r3);
    // println!("{} and {}", r1, r2); 如果加上这句就会出错
}
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn dangle() -> &String { // dangle 返回一个字符串的引用
//
//     let s = String::from("hello"); // s 是一个新字符串
//
//     &s // 返回字符串 s 的引用
// } // 这里 s 离开作用域并被丢弃。其内存被释放。
// 危险！

fn no_dangle() -> String {
    let s = String::from("hello from no_dangle");

    s
}