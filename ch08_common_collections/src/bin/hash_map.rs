use std::collections::HashMap;

fn main() {
    // 像 vector 一样，哈希 map 将它们的数据储存在堆上
    // 1. 新建一个HashMap
    // 1.1. new一个
    let mut scores: HashMap<String, u8> = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("{:?}", scores);
    // 1.2. 使用Vector的collect
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![30, 70];
    // 注意, 这里使用了iter(), 所以类型是HashMap<&String, &i32>
    // let scores: HashMap<&String, &i32> = teams.iter().zip(initial_scores.iter()).collect();
    // 这里 HashMap<_, _> 类型标注是必要的，因为 collect 有可能当成多种不同的数据结构，而除非显式指定否则 Rust 无从得知你需要的类型。
    // 但是对于键和值的类型参数来说，可以使用下划线占位，而 Rust 能够根据 vector 中数据的类型推断出 HashMap 所包含的类型。
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("{:?}", scores);

    let pairs = vec![("key1", 10), ("key2", 20)];
    let mut map: HashMap<_, _> = pairs.into_iter().collect();
    map.insert("key3", 30);
    println!("{:?}", map);

    // 2. HashMap和所有权
    // 对于像 i32 这样的实现了 Copy trait 的类型，其值可以拷贝进哈希 map。
    // 对于像 String 这样拥有所有权的值，其值将被移动而哈希 map 会成为这些值的所有者
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // 这里 field_name 和 field_value 不再有效，
    // 尝试使用它们看看会出现什么编译错误！
    // println!("{:?}", field_name);

    // 当 insert 调用将 field_name 和 field_value 移动到哈希 map 中后，将不能使用这两个绑定。
    //
    // 如果将值的引用插入哈希 map，这些值本身将不会被移动进哈希 map。但是这些引用指向的值必须至少在哈希 map 有效时也是有效的。

    // 3，访问HashMap中的值
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    // 注意这里需要使用引用&
    // pub fn get<Q: ?Sized>(&self, k: &Q) -> Option<&V>
    let result = scores.get(&String::from("Blue"));
    match result {
        Some(v) => { println!("value: {v}") }
        None => { println!("no value") }
    }
    // 遍历
    for (k, v) in scores {
        println!("{}: {}", k, v);
    }

    // 4，更新HashMap
    // 4.1. 覆盖一个值
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    // scores.insert(String::from("Blue"), 60); // 被替换成60
    println!("{:?}", scores);
    // 4.2. 只在键没有对应值时插入
    scores.entry(String::from("Blue")).or_insert(50);
    scores.entry(String::from("Red")).or_insert(50);
    println!("{:?}", scores); // {"Yellow": 50, "Red": 50, "Blue": 10}
    println!("{:?}", scores.entry(String::from("Black"))); // Entry(VacantEntry("Black"))
    println!("{:?}", scores.entry(String::from("Red"))); // Entry(OccupiedEntry { key: "Red", value: 50, .. })
    // entry，它获取我们想要检查的键作为参数。entry 函数的返回值是一个枚举，Entry，它代表了可能存在也可能不存在的值。
    // Entry 的 or_insert 方法在键对应的值存在时就返回这个值的可变引用，如果不存在则将参数作为新值插入并返回新值的可变引用。

    // 4.3. 根据旧值更新一个值
    // 计数一些文本中每一个单词分别出现了多少次
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        // 这里是因为or_insert()会返回value的可变引用（&mut V）
        let count = map.entry(word).or_insert(0);
        // 然后计数 + 1
        *count += 1;
    }
    println!("{:?}", map);
}