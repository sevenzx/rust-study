fn main() {
    // 1. 变量和可变性
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // 2. 常量
    // 常量不仅仅默认不可变，而且自始至终不可变。常量使用 const 关键字而不是 let 关键字来声明，并且值的类型必须注明
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of THREE_HOURS_IN_SECONDS is: {}", THREE_HOURS_IN_SECONDS);

    // 3. 遮蔽
    // 详情查看: https://www.rustwiki.org.cn/zh-CN/book/ch03-01-variables-and-mutability.html#%E9%81%AE%E8%94%BD
    let y = 5;
    let y = y + 1;
    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {}", y); // 12
    }
    println!("The value of y is: {}", y); // 6

    // mut 和遮蔽之间的另一个区别是，因为我们在再次使用 let 关键字时有效地创建了一个新的变量，
    // 所以我们可以改变值的类型，但重复使用相同的名称
    let spaces = "   ";
    println!("spaces is: {:?}", spaces);
    let spaces = spaces.len();
    println!("spaces is: {:?}", spaces);
}