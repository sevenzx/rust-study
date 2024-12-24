use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn use_message_passing() {
    // 1. 使用消息传递在线程间传送数据

    // Rust 中一个实现消息传递并发的主要工具是 通道（channel），Rust 标准库提供了其实现的编程概念。
    // 你可以将其想象为一个水流的通道，比如河流或小溪。如果你将诸如橡皮鸭或小船之类的东西放入其中，它们会顺流而下到达下游。
    //
    // 编程中的通道有两部分组成，一个发送者（transmitter）和一个接收者（receiver）。

    // mpsc 是 多个生产者，单个消费者（multiple producer, single consumer）的缩写。简而言之，
    // Rust 标准库实现通道的方式意味着一个通道可以有多个产生值的 发送（sending）端，但只能有一个消费这些值的 接收（receiving）端。
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("Hi");
        tx.send(val).unwrap()
        // println!("val is {}", val) 尝试在新建线程中的通道中发送完 val 值 之后 再使用它 是不被允许的
    });

    let received_msg = rx.recv().unwrap();
    println!("Got: {}", received_msg)

    // 通道的接收端有两个有用的方法：recv 和 try_recv。这里，我们使用了 recv，它是 receive 的缩写。
    // 这个方法会阻塞主线程执行直到从通道中接收一个值。一旦发送了一个值，recv 会在一个 Result<T, E> 中返回它。
    // 当通道发送端关闭，recv 会返回一个错误表明不会再有新的值到来了。
    //
    // try_recv 不会阻塞，相反它立刻返回一个 Result<T, E>：Ok 值包含可用的信息，而 Err 值代表此时没有任何消息。
    // 如果线程在等待消息过程中还有其他工作时使用 try_recv 很有用：
    // 可以编写一个循环来频繁调用 try_recv，在有可用消息时进行处理，其余时候则处理一会其他工作直到再次检查。
}

fn send_multiple_values() {
    // 2. 发送多个值并观察接收者的等待
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

fn clone_transmitter() {
    // 3. 通过克隆发送者来创建多个生产者
    // 之前我们提到了mpsc是 multiple producer, single consumer 的缩写。
    // 可以运用 mpsc 来扩展示例代码来创建向同一接收者发送值的多个线程。这可以通过克隆通道的发送端来做到，
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

fn main() {
    // use_message_passing();
    // send_multiple_values();
    clone_transmitter();

    return;
}
