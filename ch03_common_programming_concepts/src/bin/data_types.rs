fn main() {
    // 数据类型

    // 一. 标量类型
    // Rust 有 4 个基本的标量类型：整型、浮点型、布尔型和字符
    // 1. 整形
    // 长度	    有符号类型	无符号类型
    // 8 位	    i8	        u8
    // 16 位	i16	        u16
    // 32 位	i32	        u32
    // 64 位	i64	        u64
    // 128 位	i128	    u128
    // arch	    isize	    usize
    // isize 和 usize 类型取决于程序运行的计算机体系结构，
    // 在表中表示为“arch”：若使用 64 位架构系统则为 64 位，若使用 32 位架构系统则为 32 位。
    // 数字字面量还可以使用 _ 作为可视分隔符以方便读数，如 1_000，此值和 1000 相同。
    let num: u32 = 1_000;
    println!("The value of num is: {}", num);

    // 2. 浮点类型
    let float32: f32 = 3.14;
    println!("The value of float 32 is: {}", float32);
    let float64: f64 = 3.14;
    println!("The value of float 64 is: {}", float64);

    // 3. 布尔类型
    let f: bool = true;
    println!("The value of f is: {}", f);
    let f: bool = false;
    println!("The value of f is: {}", f);

    // 4. 字符类型
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("c: {}, z: {}, heart_eyed_cat: {}", c, z, heart_eyed_cat);

    // 二、复合类型
    // Rust 有两种基本的复合类型：元组（tuple）和数组（array）。
    // 1. 元组
    // 元组是将多种类型的多个值组合到一个复合类型中的一种基本方式。
    // 元组的长度是固定的：声明后，它们就无法增长或缩小。
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // 使用解构从元组中读取数据
    let (x, y, z) = tup;
    println!("x: {}, y: {}, z: {}", x, y, z);
    // 使用索引从元组中读取数据
    let x = tup.0;
    let y = tup.1;
    let z = tup.2;
    println!("x: {}, y: {}, z: {}", x, y, z);

    // 2. 数组
    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];
    dbg!(months);

    // 使用方括号编写数组的类型，其中包含每个元素的类型、分号，然后是数组中的元素数
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    dbg!(a);
    // 对于每个元素都相同的情况，还可以通过指定初始值、后跟分号和方括号中的数组长度来初始化数组
    let a = [3; 5];
    dbg!(a);
}