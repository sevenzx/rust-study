fn main() {
    flow_if();
    flow_loop();
    flow_while();
    flow_for()
}

fn flow_if() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // 这里有点类似于三元，但是{}是代码快 里面还能继续加语句
    let greater_than_zero = if number > 0 { true } else { false };
    println!("greater than zero is {}", greater_than_zero);
}

fn flow_loop() {
    // 1. 如果存在嵌套循环，break 和 continue 应用于此时最内层的循环。
    // 可以选择在一个循环上指定一个循环标签（loop label），然后将标签与 break 或 continue 一起使用
    let mut count = 0;
    'outer: loop {
        println!("count = {}", count);
        let mut remaining = 10;
        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'outer;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {}", count);

    // 2. 在用于停止循环的 break 表达式添加你想要返回的值
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}

fn flow_while() {
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
}

fn flow_for() {
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {}", element);
    }

    // Usually, iterators iterate from left to right.
    // After using rev(), an iterator will instead iterate from right to left.
    for number in (1..=3).rev() {
        println!("{}!", number);
    }
}
