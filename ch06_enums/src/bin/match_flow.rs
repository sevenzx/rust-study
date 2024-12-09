#[derive(Debug)] // 这样可以立刻看到州的名称
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
fn main() {
    // 1. 简单使用
    println!("{}", value_in_cents(Coin::Penny));
    println!("{}", value_in_cents(Coin::Nickel));
    println!("{}", value_in_cents(Coin::Dime));
    // 2. 匹配分支的另一个有用的功能是可以绑定匹配的模式的部分值。这也就是如何从枚举成员中提取值的。
    println!("{}", value_in_cents(Coin::Quarter(UsState::Alabama)));
    // 3. 匹配Option<T>
    let x: Option<i32> = Some(5);
    println!("{:?}", plus_one(x));
    println!("{:?}", plus_one(None));
    // 4. 匹配是穷尽的
    // 我们没有处理 None 的情况，所以这些代码会造成一个 bug 并不能编译。
    // fn plus_one(x: Option<i32>) -> Option<i32> {
    //     match x {
    //         Some(i) => Some(i + 1),
    //     }
    // }

    // 5. 通配模式和_占位符
    // 通配模式
    dice_game(4);
    // _占位符
    dice_game_2(5)
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None,
    }
}

fn dice_game(x: u8) {
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        // other 分支的代码通过将其传递给 println! 来使用这个变量
        other => println!("{}", other),
    }
}

fn dice_game_2(x: u8) {
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        // 我们不需要使用这个值，所以我们改动代码使用 _ 来替代变量 other
        _ => println!("you win"),
    }
}