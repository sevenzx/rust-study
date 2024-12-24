use std::sync::{Arc, Mutex};
use std::thread;

fn mutex() {
    // 1. 简单使用互斥锁
    let m = Mutex::new(5);
    {
        // 如果另一个线程拥有锁，并且那个线程 panic 了，则 lock 调用会失败。
        // 在这种情况下，没人能够再获取锁，所以这里选择 unwrap 并在遇到这种情况时使线程 panic。
        let mut num = m.lock().unwrap();
        *num = 6;
        // Mutex<T> 是一个智能指针。更准确的说，lock 调用 返回 一个叫做 MutexGuard 的智能指针。
        // 这个智能指针实现了 Deref 来指向其内部数据；其也提供了一个 Drop 实现当 MutexGuard 离开作用域时自动释放锁
        // 所以这里释放锁了
    }

    println!("m = {:?}", m)
}

fn multiple_thread() {
    // 2. 在线程间共享 Mutex<T>
    // 原子引用计数 Arc<T>
    //  Arc<T> 正是 这么一个类似 Rc<T> 并可以安全的用于并发环境的类型。字母 “a” 代表 原子性（atomic），
    // 所以这是一个原子引用计数（atomically reference counted）类型。
    // 原子性是另一类这里还未涉及到的并发原语：请查看标准库中 std::sync::atomic 的文档来获取更多细节。
    // 其中的要点就是：原子性类型工作起来类似原始类型，不过可以安全的在线程间共享。
    let counter = Arc::new(Mutex::new(0));
    let mut handlers = vec![];

    for _ in 0..10 {
        // 启用多所有权
        let counter = Arc::clone(&counter);
        let handler = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handlers.push(handler);
    }

    for handler in handlers {
        handler.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

fn main() {
    // mutex();
    multiple_thread();

    return;
}
