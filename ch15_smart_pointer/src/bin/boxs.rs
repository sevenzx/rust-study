enum List {
    Cons(i32, Box<List>),
    Nil,
}
#[test]
fn cons_list() {
    use crate::List::{Cons, Nil};

    let list = Cons(1,
                    Box::new(Cons(2,
                                  Box::new(Cons(3,
                                                Box::new(Nil))))));
}

fn main() {
    // 最简单直接的智能指针是 box，其类型是 Box<T>。 box 允许你将一个值放在堆上而不是栈上。留在栈上的则是指向堆数据的指针。
    // 除了数据被储存在堆上而不是栈上之外，box 没有性能损失。不过也没有很多额外的功能。它们多用于如下场景：
    //
    // 1. 当有一个在编译时未知大小的类型，而又想要在需要确切大小的上下文中使用这个类型值的时候
    // 2. 当有大量数据并希望在确保数据不被拷贝的情况下转移所有权的时候
    // 3. 当希望拥有一个值并只关心它的类型是否实现了特定 trait 而不是其具体类型的时候
    let b = Box::new(5);
    println!("b = {}", b);
}