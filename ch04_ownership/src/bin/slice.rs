fn main() {
    // 另一个没有所有权的数据类型是 slice。slice 允许你引用集合中一段连续的元素序列，而不用引用整个集合。
    let my_string = String::from("hello world");

    // `first_word` 接受 `String` 的切片，无论是部分还是全部
    let word = first_word(&my_string[0..6]);
    println!("word: {}", word);
    let word = first_word(&my_string[..]);
    println!("word: {}", word);
    // `first_word` 也接受 `String` 的引用，
    // 这等同于 `String` 的全部切片
    let word = first_word(&my_string);
    println!("word: {}", word);

    let my_string_literal = "hello world";

    // `first_word` 接受字符串字面量的切片，无论是部分还是全部
    let word = first_word(&my_string_literal[0..6]);
    println!("word: {}", word);
    let word = first_word(&my_string_literal[..]);
    println!("word: {}", word);

    // 因为字符串字面值 就是 字符串 slice，
    // 这样写也可以，即不使用 slice 语法！
    let word = first_word(my_string_literal);
    println!("word: {}", word);

    other_slice()
}

fn first_word(s: &str) -> &str {
    // “字符串 slice” 的类型声明写作 &str
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
fn other_slice() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let slice: &[i32] = &a[1..3];
    println!("slice: {:?}", slice);
}