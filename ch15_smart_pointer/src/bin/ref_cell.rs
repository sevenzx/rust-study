use std::cell::RefCell;
use std::rc::Rc;
use crate::List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}


pub trait Messenger {
    fn send(&self, msg: &str);
}
pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }
    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage_of_max = self.value as f64 / self.max as f64;
        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger { sent_messages: RefCell::new(vec![]) }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
fn main() {
    // 1.
    // 类似于 Rc<T>，RefCell<T> 只能用于单线程场景。如果尝试在多线程上下文中使用RefCell<T>，会得到一个编译错误。
    //
    // 如下为选择 Box<T>，Rc<T> 或 RefCell<T> 的理由：
    //
    // Rc<T> 允许相同数据有多个所有者；Box<T> 和 RefCell<T> 有单一所有者。
    // Box<T> 允许在编译时执行不可变或可变借用检查；Rc<T>仅允许在编译时执行不可变借用检查；RefCell<T> 允许在运行时执行不可变或可变借用检查。
    // 因为 RefCell<T> 允许在运行时执行可变借用检查，所以我们可以在即便 RefCell<T> 自身是不可变的情况下修改其内部的值。


    // 2. 内部可变性用例 测试中MockMessenger
    // 现在 sent_messages 字段的类型是 RefCell<Vec<String>> 而不是 Vec<String>。
    // 在 new 函数中新建了一个 RefCell<Vec<String>> 实例替代空 vector。
    //
    // 对于 send 方法的实现，第一个参数仍为 self 的不可变借用，这是符合方法定义的。
    // 我们调用 self.sent_messages 中 RefCell 的 borrow_mut 方法来获取 RefCell 中值的可变引用，这是一个 vector。
    // 接着可以对 vector 的可变引用调用 push 以便记录测试过程中看到的消息。
    //
    // 最后必须做出的修改位于断言中：为了看到其内部 vector 中有多少个项，
    // 需要调用 RefCell 的 borrow 以获取 vector 的不可变引用。

    // 3. RefCell<T> 在运行时记录借用
    // 当创建不可变和可变引用时，我们分别使用 & 和 &mut 语法。
    // 对于 RefCell<T> 来说，则是 borrow 和 borrow_mut 方法，这属于 RefCell<T> 安全 API 的一部分。
    // borrow 方法返回 Ref<T> 类型的智能指针，borrow_mut 方法返回 RefMut 类型的智能指针。
    // 这两个类型都实现了 Deref，所以可以当作常规引用对待。
    // RefCell<T> 记录当前有多少个活动的 Ref<T> 和 RefMut<T> 智能指针。
    // 每次调用 borrow，RefCell<T> 将活动的不可变借用计数加一。当 Ref<T> 值离开作用域时，不可变借用计数减一。
    // 就像编译时借用规则一样，RefCell<T> 在任何时候只允许有多个不可变借用或一个可变借用。

    // 4. 结合 Rc<T> 和 RefCell<T> 来拥有多个可变数据所有者
    // RefCell<T> 的一个常见用法是与 Rc<T> 结合。回忆一下 Rc<T> 允许对相同数据有多个所有者，不过只能提供数据的不可变访问。
    // 如果有一个储存了 RefCell<T> 的 Rc<T> 的话，就可以得到有多个所有者 并且 可以修改的值了
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);

}