use std::time;
use futures::executor::block_on;

#[test]
fn async1() {
    async fn hello_world() {
        hello_cat().await;
        println!("hello, world!");
    }

    async fn hello_cat() {
        println!("hello, kitty!");
    }

    let future = hello_world();
    block_on(future)
}

#[test]
fn async2(){
    async fn sing() {
        async_std::task::sleep(time::Duration::from_secs(1)).await;
        println!("sing! {:?}", std::thread::current().id());
    }

    async fn dance() {
        println!("dance! {:?}", std::thread::current().id())
    }


    async fn async_main() {
        let sing = sing();
        let dance = dance();

        // `join!`可以并发的处理和等待多个`Future`，若`sing Future`被阻塞(async_std::task::sleep(time::Duration::from_secs(1)).await;)
        // 那`dance Future`可以拿过线程的所有权继续执行。若`dance`也变成阻塞状态，那`sing`又可以再次拿回线程所有权，继续执行。
        // 若两个都被阻塞，那么`async main`会变成阻塞状态，然后让出线程所有权，并将其交给`main`函数中的`block_on`执行器
        futures::join!(sing, dance);
    }

    block_on(async_main())
}

fn main() {
    println!("Hello, world!");
}
