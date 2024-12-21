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


    // 闭包会捕获其环境
    let x = 4;
    // 这里，即便 x 并不是 equal_to_x 的一个参数，equal_to_x 闭包也被允许使用变量 x，因为它与 equal_to_x 定义于相同的作用域。
    let equal_to_x = |z| z == x;
    let y = 4;
    assert!(equal_to_x(y));


    // 在 Rust 中，闭包是一种可以捕获其定义时环境中变量的匿名函数。理解闭包如何捕获环境中的值及其对应的内存开销，可以帮助我们更有效地使用 Rust 的特性。
    // 闭包的捕获方式
    // 闭包可以通过三种方式捕获环境中的值，这些方式对应于函数参数的三种获取方式：
    // 获取所有权：闭包通过获取值的所有权来捕获变量。此时，闭包只能被调用一次，因为它消耗了捕获的变量。实现这一特性的 trait 是 FnOnce。
    // 可变借用：闭包以可变借用的方式捕获变量，这样它可以修改这些变量。实现这一特性的 trait 是 FnMut。
    // 不可变借用：闭包以不可变借用的方式捕获变量，因此它只能读取这些变量，而不能修改。实现这一特性的 trait 是 Fn。
    // Trait 解析
    // FnOnce：允许闭包消费其捕获的变量，调用后无法再次使用这些变量。
    // FnMut：允许闭包可变借用其捕获的变量，可以多次调用并修改这些变量。
    // Fn：允许闭包不可变借用其捕获的变量，可以多次调用，但不能修改。
    // 闭包与内存开销
    // 当闭包捕获环境中的值时，会引入额外的内存开销。例如，如果一个闭包需要持有一个大数据结构的所有权，
    // 那么在调用这个闭包时就会消耗相应的内存资源。如果不需要闭包捕获环境中的值，使用普通函数将不会产生这样的开销。
    // 使用 move 关键字
    // 如果希望强制闭包获取其使用的环境值的所有权，可以在定义闭包时使用 move 关键字。这在需要将数据传递到新线程或返回闭包时特别有用。

    //
    // let x = vec![1, 2, 3];
    // let equal_to_x = move |z| z == x;
    // println!("can't use x here: {:?}", x);
    // let y = vec![1, 2, 3];
    // assert!(equal_to_x(y));
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
