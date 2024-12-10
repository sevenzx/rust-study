fn main() {
    // 1. 新建Vector
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];

    // 2. 更新Vector
    let mut v = vec![1, 2, 3];
    v.push(4);
    v.push(5);
    v.push(6);
    println!("{:?}", v);

    // 3. 读取Vector, 有两种方式
    // 3.1. 索引
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    // 3.2. get
    match v.get(3) {
        Some(v) => println!("The value is {}", v),
        None => println!("none"),
    }

    // 3.3.
    // 代码看起来应该能够运行：为什么第一个元素的引用会关心 vector 结尾的变化？不能这么做的原因是由于 vector 的工作方式：
    // 在 vector 的结尾增加新元素时，在没有足够空间将所有所有元素依次相邻存放的情况下，
    // 可能会要求分配新内存并将老的元素拷贝到新的空间中。这时，第一个元素的引用就指向了被释放的内存。借用规则阻止程序陷入这种状况。
    // let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0];
    // v.push(6); // 报错啦 mutable borrow occurs here
    // println!("The first element is: {}", first);

    // 这样就正常了
    let mut v = vec![1, 2, 3, 4, 5];
    let first = v[0];
    v.push(6);
    println!("The first element is: {}", first);

    // 4. 遍历 vector 中的元素
    for x in &v {
        println!("The x is {}", x);
    }
    // 也可以遍历改变它们
    for x in &mut v {
        *x *= 2
    }
    dbg!(v);

    // 5. 使用枚举来存储多种类型
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];


    dbg!(row);
}