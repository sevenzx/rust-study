fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = add_one(5);
    println!("The value of x is: {}", x);

    // 注意语句和表达式
    // 语句（statement）是执行一些操作但不返回值的指令。表达式（expression）计算并产生一个值。
    let y = {
        let x = 3; // 语句
        x + 1           // 表达式
    };

    println!("The value of y is: {}", y);
}

fn add_one(x: i32) -> i32 {
    // 函数可以向调用它的代码返回值。我们并不对返回值命名，但要在箭头（->）后声明它的类型。
    // 在 Rust 中，函数的返回值等同于函数体最后一个表达式的值。
    // 使用 return 关键字和指定值，可从函数中提前返回；但大部分函数隐式的返回最后的表达式。
    x + 1 // 最后一个表达式
}