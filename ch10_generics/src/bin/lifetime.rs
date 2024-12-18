fn main() {
    // let r;
    //
    // {
    //     let x = 5;
    //     r = &x;
    // }
    //
    // println!("r: {}", r);

    // error[E0597]: `x` does not live long enough
    //  --> src/bin/lifetime.rs:6:13
    //   |
    // 5 |         let x = 5;
    //   |             - binding `x` declared here
    // 6 |         r = &x;
    //   |             ^^ borrowed value does not live long enough
    // 7 |     }
    //   |     - `x` dropped here while still borrowed
    // 8 |
    // 9 |     println!("r: {}", r);
    //   |                       - borrow later used here


    // 4. 静态生命周期
    // 这里有一种特殊的生命周期值得讨论：'static，其生命周期能够存活于整个程序期间。
    // 所有的字符串字面量都拥有 'static 生命周期，我们也可以选择像下面这样标注出来
    {
        let s: &'static str = "I have a static lifetime.";
        println!("Hello, {}!", s);
    }
    // println!("Hello, {}!", s); // 这里会编译报错 not found in this scope
}


// 2. 函数中的泛型生命周期
// 错误写法
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }
// 提示文本揭示了返回值需要一个泛型生命周期参数，因为 Rust 并不知道将要返回的引用是指向 x 或 y。事实上我们也不知道，
// 因为函数体中 if 块返回一个 x 的引用而 else 块返回一个 y 的引用！
// 当我们定义这个函数的时候，并不知道传递给函数的具体值，所以也不知道到底是 if 还是 else 会被执行。
// 我们也不知道传入的引用的具体生命周期，借用检查器自身同样也无法确定，因为它不知道 x 和 y 的生命周期是如何与返回值的生命周期相关联的。
// 为了修复这个错误，我们将增加泛型生命周期参数来定义引用间的关系以便借用检查器可以进行分析。

// &i32        // 引用
// &'a i32     // 带有显式生命周期的引用
// &'a mut i32 // 带有显式生命周期的可变引用
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
// 现在函数签名表明对于某些生命周期 'a，函数会获取两个参数，他们都是与生命周期 'a 存在的一样长的字符串 slice。
// 函数会返回一个同样也与生命周期 'a 存在的一样长的字符串 slice。
// 它的实际含义是 longest 函数返回的引用的生命周期与传入该函数的引用的生命周期的较小者一致。 划重点 较小者。

// 3. 结构体定义中的生命周期标注
// 这个结构体有一个字段，part，它存放了一个字符串 slice，这是一个引用。类似于泛型参数类型，
// 必须在结构体名称后面的尖括号中声明泛型生命周期参数，以便在结构体定义中使用生命周期参数。
// 这个标注意味着 ImportantExcerpt 的实例不能比其 part 字段中的引用存在的更久。
struct ImportantExcerpt<'a> {
    part: &'a str,
}
// fn main() {
//     let novel = String::from("Call me Ishmael. Some years ago...");
//     let first_sentence = novel.split('.')
//         .next()
//         .expect("Could not find a '.'");
//     let i = ImportantExcerpt { part: first_sentence };
// }
// 这里的 main 函数创建了一个 ImportantExcerpt 的实例，它存放了变量 novel 所拥有的 String 的第一个句子的引用。
// novel 的数据在 ImportantExcerpt 实例创建之前就存在。另外，直到 ImportantExcerpt 离开作用域之后
// novel 都不会离开作用域，所以 ImportantExcerpt 实例中的引用是有效的。
