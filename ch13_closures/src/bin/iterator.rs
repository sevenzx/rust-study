#[test]
fn use_iterator() {
    // 迭代器模式允许你对一个序列的项进行某些处理。迭代器（iterator）负责遍历序列中的每一项和决定序列何时结束的逻辑。
    // 当使用迭代器时，我们无需重新实现这些逻辑。
    //
    // 在 Rust 中，迭代器是 惰性的（lazy），这意味着在调用方法使用迭代器之前它都不会有效果。
    // 1. 在一个 for 循环中使用迭代器
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    // for i in 0..v1.len() {
    //     println!("{}", v1.get(i).unwrap());
    // }
    for val in v1_iter {
        println!("Got: {}", val);
    }
}

#[test]
fn iter_next() {
    // 2. Iterator trait 和 next 方法
    // 注意 v1_iter 需要是可变的：在迭代器上调用 next 方法改变了迭代器中用来记录序列位置的状态。
    // 换句话说，代码 消费（consume）了，或使用了迭代器。每一个 next 调用都会从迭代器中消费一个项。
    // 使用 for 循环时无需使 v1_iter 可变因为 for 循环会获取 v1_iter 的所有权并在后台使 v1_iter 可变。
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
    // 另外需要注意到从 next 调用中得到的值是 vector 的不可变引用。iter 方法生成一个不可变引用的迭代器。
    // 如果我们需要一个获取 v1 所有权并返回拥有所有权的迭代器，则可以调用 into_iter 而不是 iter。
    // 类似的，如果我们希望迭代可变引用，则可以调用 iter_mut 而不是 iter。
    // .iter()
    // 功能: 返回集合的不可变引用迭代器。
    // 所有权: 该方法不会转移集合的所有权，只是借用元素的引用。
    // 返回类型: 迭代器返回的是元素的不可变引用，即Some(&T)。

    // .iter_mut()
    // 功能: 返回集合的可变引用迭代器。
    // 所有权: 同样不会转移集合的所有权，但允许对元素进行修改。
    // 返回类型: 迭代器返回的是元素的可变引用，即Some(&mut T)。
    let mut nums = vec![1, 2, 3, 4];
    let iter_mut = nums.iter_mut();
    for num in iter_mut {
        *num += 1; // 修改每个元素
    }
    println!("{:?}", nums); // 输出: [2, 3, 4, 5]
    // .into_iter()
    // 功能: 将集合转移所有权并生成值迭代器。
    // 所有权: 调用该方法后，原始集合将无法再使用，因为所有权已被转移。
    // 返回类型: 迭代器返回的是元素本身，即Some(T)。
    // 总结
    // 使用.iter()时，你可以读取集合中的值，但不能修改它们。
    // 使用.iter_mut()时，你可以读取并修改集合中的值。
    // 使用.into_iter()时，你将获取集合中的值，并且原始集合将不再可用。
    let nums = vec![1, 2, 3, 4];
    let into_iter = nums.into_iter();
    for num in into_iter {
        println!("{}", num); // 输出: 1 2 3 4
    }
    // println!("{:?}", nums); // 此行代码会报错，因为nums的所有权已经转移
}

#[test]
fn consume_iter() {
    // 3. 消费迭代器
    // Iterator trait 有一系列不同的由标准库提供默认实现的方法；你可以在 Iterator trait 的标准库 API 文档中找到所有这些方法。
    // 一些方法在其定义中调用了 next 方法，这也就是为什么在实现 Iterator trait 时要求实现 next 方法的原因。
    //
    // 这些调用 next 方法的方法被称为 消费适配器（consuming adaptors），因为调用他们会消耗迭代器。
    // 一个消费适配器的例子是 sum 方法。这个方法获取迭代器的所有权并反复调用 next 来遍历迭代器，因而会消费迭代器。
    // 当其遍历每一个项时，它将每一个项加总到一个总和并在迭代完成时返回总和。
    let v = vec![1, 2, 3];
    let iter = v.iter();
    let total: i32 = iter.sum();
    println!("Sum: {:?}", total);

    // iter 不再可用了
    // for num in iter {
    //     println!("{}", num);
    // }

    // 4. Iterator.map
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<i32> = v1.iter().map(|x| x + 1).collect();
    println!("{:?}", v2);

    // 5. Iterator.filter
    let v3: Vec<i32> = vec![1, 2, 3, 4, 5, 6];
    let v4: Vec<i32> = v3.into_iter().filter(|x| x % 3 == 0).collect();
    println!("{:?}", v4);
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn calling_next_directly() {
    // 6. 实现 Iterator trait 来创建自定义迭代器
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}


#[test]
fn using_other_iterator_trait_methods() {
    // 例如，出于某种原因我们希望获取 Counter 实例产生的值，将这些值与另一个 Counter 实例在省略了第一个值之后产生的值配对，
    // 将每一对值相乘，只保留那些可以被三整除的结果，然后将所有保留的结果相加
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    println!("The sum is {}", sum);
}
fn main() {}