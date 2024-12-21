use std::collections::HashMap;
use std::thread;
use std::time::Duration;

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: HashMap<u32, u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }
    fn value(&mut self, arg: u32) -> u32 {
        match self.value.get(&arg) {
            Some(v) => *v,
            None => {
                // 在 Rust 中，使用闭包或函数指针时，确实需要用括号()将其括起来。
                // 这是因为 Rust 的语法要求在调用函数时明确地表明你是在调用一个函数或闭包，而不是引用它。
                let v = (self.calculation)(arg);
                self.value.insert(arg, v);
                v
            }
        }
    }
}

fn main() {
    // Rust 的 闭包（closures）是可以保存进变量或作为参数传递给其他函数的匿名函数。可以在一个地方创建闭包，
    // 然后在不同的上下文中执行闭包运算。不同于函数，闭包允许捕获调用者作用域中的值。我们将展示闭包的这些功能如何复用代码和自定义行为。

    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number,
    );
}

fn generate_workout(intensity: u32, random_number: u32) {
    // 闭包的定义
    // 闭包的定义以一对竖线（|）开始，在竖线中指定闭包的参数；之所以选择这个语法是因为它与 Smalltalk 和 Ruby 的闭包定义类似。
    // 这个闭包有一个参数 num；如果有多于一个参数，可以使用逗号分隔，比如 |param1, param2|。

    // 闭包不要求像 fn 函数那样在参数和返回值上注明类型。 闭包通常很短，并只关联于小范围的上下文而非任意情境。
    // 在这些有限制的上下文中，编译器能可靠的推断参数和返回值的类型， 类似于它是如何能够推断大部分变量的类型一样。
    // 类似于变量，如果相比严格的必要性你更希望增加明确性并变得更啰嗦，可以选择增加类型标注
    // let expensive_closure = |num: u32| -> u32 {
    //     println!("calculating slowly...");
    //     thread::sleep(Duration::from_secs(2));
    //     num
    // };
    // let expensive_closure = |num| {
    //     println!("calculating slowly...");
    //     thread::sleep(Duration::from_secs(2));
    //     num
    // };

    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    expensive_result.value(2);

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            // 调用闭包类似于调用函数
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}
