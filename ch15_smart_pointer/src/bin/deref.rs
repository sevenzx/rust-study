use std::ops::Deref;

// 元组结构体
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // 指的是元组结构体中的第一个字段
        &self.0
    }
}

fn main() {
    // 实现 Deref trait 允许我们重载 解引用运算符（dereference operator）*（与乘法运算符或通配符相区别）。
    // 通过这种方式实现 Deref trait 的智能指针可以被当作常规引用来对待，可以编写操作引用的代码并用于智能指针。

    // 1. 通过解引用运算符追踪指针的值
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // 2. 像引用一样使用 Box<T>
    let x = 5;
    let y = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    // 3. 自定义智能指针
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y); // *y在rust的底层其实是  *(y.deref())

    // 4. 函数和方法的隐式解引用强制转换
    // 解引用强制转换（deref coercions）是 Rust 在函数或方法传参上的一种便利。解引用强制转换只能工作在实现了 Deref trait 的类型上。
    // 解引用强制转换将一种类型（A）隐式转换为另外一种类型（B）的引用，因为 A 类型实现了 Deref trait，并且其关联类型是 B 类型。
    // 比如，解引用强制转换可以将 &String 转换为 &str，因为类型 String 实现了 Deref trait 并且其关联类型是 str。 源码如下
    // #[stable(feature = "rust1", since = "1.0.0")]
    // impl ops::Deref for String {
    //     type Target = str;
    //
    //     #[inline]
    //     fn deref(&self) -> &str {
    //         self.as_str()
    //     }
    // }

    let message = MyBox::new(String::from("Rust"));
    // 这里就是隐式解引用强制转换 &MyBox<String> -> &String -> &str
    hello(&message);
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}
