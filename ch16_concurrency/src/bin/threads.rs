use std::thread;
use std::time::Duration;

#[test]
fn create_a_thread() {
    // 1. 使用 spawn 创建新线程
    // 为了创建一个新线程，需要调用 thread::spawn 函数并传递一个闭包，并在其中包含希望在新线程运行的代码。
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // 2. 使用 join 等待线程结束
    // thread::spawn 的返回值类型是 JoinHandle。JoinHandle 是一个拥有所有权的值，当对其调用 join 方法时，它会等待其线程结束。
    handle.join().unwrap();
}

#[test]
fn use_move_closure() {
    // 3. 线程与 move 闭包
    let v = vec![1, 2, 3];

    // 通过在闭包之前增加 move 关键字，我们强制闭包获取其使用的值的所有权
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    // drop(v);
    // 35 |     drop(v);
    //    |          ^ value used here after move


    handle.join().unwrap();
}


fn main() {}