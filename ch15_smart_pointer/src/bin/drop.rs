struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    // 详情查看 https://www.rustwiki.org.cn/zh-CN/book/ch15-03-drop.html
    // 指定在值离开作用域时应该执行的代码的方式是实现 Drop trait。Drop trait
    // 要求实现一个叫做 drop 的方法，它获取一个 self 的可变引用。
    let c = CustomSmartPointer { data: String::from("my stuff") };
    let d = CustomSmartPointer { data: String::from("other stuff") };
    println!("CustomSmartPointers created.");
    // 主动drop
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}