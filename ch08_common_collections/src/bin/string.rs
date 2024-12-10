fn main() {
    // 1. 新建字符串
    let mut s = String::new();
    let data = "initial contents";
    let s = data.to_string();
    let s = "initial contents".to_string();
    let s = String::from("initial contents");

    // 2. 更新字符串
    // 2.1. 使用 push_str 和 push 附加字符串
    let mut s = String::from("foo");
    s.push_str("bar");
    s.push('!');
    println!("{}", s);

    // 2.2. 使用+运算符或 format！宏拼接字符串
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // 注意 s1 被移动了，不能继续使用
    println!("{}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3); // 这里没有被移动可以继续使用
    println!("{},{},{},{}", s1, s2, s3, s);

    // 3. 索引字符串
    let s = String::from("hello");
    // println!("{}", s[0]); // 会报错 Rust 的字符串不支持索引
    // String 是一个 Vec<u8> 的封装。
    println!("{}", s.len()); // 5
    println!("{}", String::from("Здравствуйте").len()); // 不是12 而是24
    // Returns the length of this String, in bytes, not chars or graphemes.
    // In other words, it might not be what a human considers the length of the string.

    // 4. 字符串slice
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("{}", s);          // Зд
    // let s = &hello[0..1]; // 运行时会 panic，就跟访问 vector 中的无效索引时一样

    println!("====");
    // 5. 遍历字符串的方法
    for c in "नमस्ते".chars() {
        println!("{}", c)
    }
    println!("====");
    for b in "नमस्ते".bytes() {
        println!("{}", b)
    }
}