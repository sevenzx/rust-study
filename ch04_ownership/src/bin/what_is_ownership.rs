fn main() {
    // 首先，让我们看一下所有权的规则。当我们通过举例说明时，请谨记这些规则：
    //
    // 1. Rust 中的每一个值都有一个被称为其 所有者（owner）的变量。
    // 2. 值在任一时刻有且只有一个所有者。
    // 3. 当所有者（变量）离开作用域，这个值将被丢弃。

    // 1. 作用域
    {
        let s = String::from("hello"); // 从此处起，s 开始有效

        // 使用 s
    }   // 此作用域已结束，
    // s 不再有效
    // 这是一个将 String 需要的内存返回给分配器的很自然的位置：当 s 离开作用域的时候。
    // 当变量离开作用域，Rust 为我们调用一个特殊的函数。这个函数叫做 drop，
    // 在这里 String 的作者可以放置释放内存的代码。Rust 在结尾的 } 处自动调用 drop。

    // 2. 字符串字面值 vs String类型
    // 这是一个字符串字面值（string literal）
    // 它们存储在静态内存中，所以很高效。
    // 适合用来表示那些不会改变的文本信息，比如程序中的固定消息、错误提示等
    let s = "hello";
    // 尝试修改字符串字面值会导致编译错误
    // s[0] = 'H';

    // 创建的是一个String类型的可变字符串。
    // String类型是一个堆分配（heap - allocated）的数据结构
    let s = String::from("hello");

    // 3. 变量与数据交互的方式
    // 3.1. 移动Move
    let s1 = String::from("hello");
    // let s2 = s1;
    // println!("{}, world!", s1);
    // 像上方这样38行会报错。
    // 字符串String类型是一个拥有堆上数据的类型。String类型在内存中存储的方式是，它在栈上有一个结构体。
    // 这个结构体包含了3部分: 一个指向堆上存放字符串内容内存的指针，一个长度，和一个容量。
    // 当s2 = s1时，Rust 会移动s1的堆内存数据的所有权到s2。这就意味着s1之后就不能再被访问了，因为它已经没有了对数据的所有权。
    // 上面的例子可以解读为 s1 被 移动 到了 s2 中。

    // 3.2. 克隆Clone
    // 如果我们 确实 需要深度复制 String 中堆上的数据，而不仅仅是栈上的数据，可以使用一个叫做 clone 的通用函数。
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    // 3.3. 只在栈上的数据: 拷贝Copy
    let x = 5;
    let y = x;
    // “将 5 绑定到 x；接着生成一个值 x 的拷贝并绑定到 y”。
    // 现在有了两个变量，x 和 y，都等于 5。
    // 这也正是事实上发生了的，因为整数是有已知固定大小的简单值，所以这两个 5 被放入了栈中。
    // 详情查看: https://www.rustwiki.org.cn/zh-CN/book/ch04-01-what-is-ownership.html#%E5%8F%AA%E5%9C%A8%E6%A0%88%E4%B8%8A%E7%9A%84%E6%95%B0%E6%8D%AE%E6%8B%B7%E8%B4%9D

    // 4. 所有权与函数
    ownership_and_functions();
    // 5. 返回值与作用域
    return_values_and_scope();
    // 变量的所有权总是遵循相同的模式：将值赋给另一个变量时移动它。
    // 当持有堆中数据值的变量离开作用域时，其值将通过 drop 被清理掉，除非数据被移动为另一个变量所有。
    // 在每一个函数中都获取所有权并接着返回所有权有些啰嗦。如果我们想要函数使用一个值但不获取所有权该怎么办呢？
    // 如果我们还要接着使用它的话，每次都传进去再返回来就有点烦人了，除此之外，我们也可能想返回函数体中产生的一些数据。
    // 我们可以使用元组来返回多个值。
    use_tuple()
    // 但是这未免有些形式主义，而且这种场景应该很常见。幸运的是，Rust 对此提供了一个功能，叫做 引用（references）。
    // 查看references_and_borrowing.rs
}

fn ownership_and_functions() {
    // 所有权与函数
    let s = String::from("hello");  // s 进入作用域

    takes_ownership(s);             // s 的值移动到函数里 ...
    // ... 所以到这里不再有效
    // println!("s = {}", s); // 会报错

    let x = 5;                      // x 进入作用域

    makes_copy(x);                  // x 应该移动函数里，
    // 但 i32 是 Copy 的，所以在后面可继续使用 x
    println!("x = {}", x)
} // 这里, x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
// 所以不会有特殊操作

fn takes_ownership(some_string: String) { // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

fn makes_copy(some_integer: i32) { // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。不会有特殊操作


fn return_values_and_scope() {
    // 返回值与作用域
    let s1 = gives_ownership();         // gives_ownership 将返回值
    // 移给 s1
    println!("s1 = {}", s1);

    let s2 = String::from("hello");     // s2 进入作用域

    let s3 = takes_and_gives_back(s2);  // s2 被移动到
    // takes_and_gives_back 中,
    // 它也将返回值移给 s3
    println!("s3 = {}", s3);
} // 这里, s3 移出作用域并被丢弃。s2 也移出作用域，但已被移走，
// 所以什么也不会发生。s1 移出作用域并被丢弃


fn gives_ownership() -> String {           // gives_ownership 将返回值移动给
    // 调用它的函数

    let some_string = String::from("yours"); // some_string 进入作用域

    some_string                              // 返回 some_string 并移出给调用的函数
}

// takes_and_gives_back 将传入字符串并返回该值
fn takes_and_gives_back(a_string: String) -> String { // a_string 进入作用域

    a_string  // 返回 a_string 并移出给调用的函数
}

fn use_tuple() {
    let s1 = String::from("hello, world~!");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() 返回字符串的长度

    (s, length)
}