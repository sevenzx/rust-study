use std::rc::Rc;
use crate::List::{Cons, Nil};

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn main() {
    // 为了启用多所有权，Rust 有一个叫做 Rc<T> 的类型。其名称为 引用计数（reference counting）的缩写。
    // 引用计数意味着记录一个值引用的数量来知晓这个值是否仍在被使用。
    // 如果某个值有零个引用，就代表没有任何有效引用并可以被清理。
    //
    // 可以将其想象为客厅中的电视。当一个人进来看电视时，他打开电视。其他人也可以进来看电视。
    // 当最后一个人离开房间时，他关掉电视因为它不再被使用了。
    // 如果某人在其他人还在看的时候就关掉了电视，正在看电视的人肯定会抓狂的！

    // 1. 使用 Rc<T> 共享数据
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));

    // 2. 克隆 Rc<T> 会增加引用计数
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a)); // 1
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a)); // 2
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a)); // 3
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a)); // 2
}