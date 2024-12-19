#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}", name)
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {}.",
                   value);
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}.",
                   value);
        }

        Guess { value }
    }
}

// #[cfg(test)]指示Rust编译器仅在执行cargo test命令时编译和运行被标记的模块。这意味着在运行cargo build时，
// 这些测试代码将不会被包含在编译结果中，从而节省了编译时间和减少了生成的可执行文件的大小
#[cfg(test)]
mod tests {
    // 注意在 tests 模块中新增加了一行：use super::*;。tests 是一个普通的模块，它遵循第 7 章 “路径用于引用模块树中的项”
    // 部分介绍的可见性规则。因为这是一个内部模块，要测试外部模块中的代码，需要将其引入到内部模块的作用域中。
    // 这里选择使用 glob 全局导入，以便在 tests 模块中使用所有在外部模块定义的内容。
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 3 };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value is `{}`", result
        );
    }

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        // u上面 expected 是期望panic的子串
        Guess::new(200);
    }

    #[test]
    fn result() -> Result<(), String> {
        if 2 + 1 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}

fn main() {
    // 1. 尝试运行测试œ
    // 运行 cargo test
    // ➜ cargo test
    //    Compiling ch11-tests v0.1.0 (/Users/bytedance/Desktop/study-project/rust/rust-study/ch11-tests)
    //     Finished `test` profile [unoptimized + debuginfo] target(s) in 0.13s
    //      Running unittests src/main.rs (target/debug/deps/ch11_tests-648909ff93a1f655)
    //
    // running 0 tests
    //
    // test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
    //
    //      Running unittests src/bin/writing_test.rs (target/debug/deps/writing_test-0a00e6fb527a76ee)
    //
    // running 1 test
    // test tests::it_works ... ok
    //
    // test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

    // 2. 使用assert!宏来检查结果
    // assert! 宏由标准库提供，在希望确保测试中一些条件为 true 时非常有用。需要向 assert! 宏提供一个求值为布尔值的参数。
    // 如果值是 true，assert! 什么也不做，同时测试会通过。如果值为 false，assert! 调用 panic! 宏，这会导致测试失败。
    // assert! 宏帮助我们检查代码是否以期望的方式运行。

    // 3. 使用 assert_eq! 和 assert_ne! 宏来测试相等

    // 4. 自定义失败信息

    // 5. 使用 should_panic 检查 panic
    // 除了检查代码是否返回期望的正确的值之外，检查代码是否按照期望处理错误也是很重要的。
    // 可以通过对函数增加另一个属性 should_panic 来实现这些。这个属性在函数中的代码 panic 时会通过，而在其中的代码没有 panic 时失败。

    //  6. 将 Result<T, E> 用于测试
}