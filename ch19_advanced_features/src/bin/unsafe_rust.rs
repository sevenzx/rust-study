#[test]
fn raw_pointer() {
    // 一. 使用unsate 解引用裸指针
    // 1. 如何从引用同时创建不可变和可变裸指针
    // 注意这里没有引入 unsafe 关键字。可以在安全代码中 创建 裸指针，只是不能在不安全块之外 解引用 裸指针
    let mut num = 5;
    let r1 = &num as *const i32; // 不可变裸指针
    let r2 = &mut num as *mut i32; // 可变裸指针

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}

#[test]
fn call_unsafe_function() {
    // 二. 调用不安全函数或方法
    unsafe fn dangerous() {
        println!("dangerous function");
    }

    unsafe {
        dangerous();
    }
}

#[test]
fn create_a_safe_abstraction_over_unsafe_code() {
    // 三. 创建不安全代码的安全抽象
    fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = slice.len();
        // as_mut_ptr 方法返回一个裸指针，指向 slice 的第一个元素
        let ptr = slice.as_mut_ptr();

        assert!(mid <= len);

        unsafe {
            (
                // slice::from_raw_parts_mut 函数是不安全的因为它获取一个裸指针，并必须确信这个指针是有效的。
                std::slice::from_raw_parts_mut(ptr, mid),
                // add 方法返回一个裸指针，指向给定的裸指针偏移 offset 个元素的位置
                std::slice::from_raw_parts_mut(ptr.add(mid), len - mid),
            )
        }
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];
    let (a, b) = v.split_at_mut(3);
    println!("a is: {:?}", a);
    println!("b is: {:?}", b);
    println!("v is: {:?}", v);
}

#[test]
fn call_external_code() {
    // 四. 使用外部代码
    // 有时你的 Rust 代码可能需要与其他语言编写的代码交互。为此 Rust 有一个关键字，extern，有助于创建和使用 外部函数接口
    // 1. 使用 extern 关键字和调用外部函数
    extern "C" {
        fn abs(input: i32) -> i32;
    }

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

// 也可以使用 extern 来创建一个允许其他语言调用 Rust 函数的接口。不同于 extern 块，就在 fn 关键字之前增加 extern 关键字并指定所用到的 ABI。
// 还需增加 #[no_mangle] 标注来告诉 Rust 编译器不要 mangle 此函数的名称。
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

#[test]
fn access_static_variable() {
    // 五. 访问或修改可变静态变量
    static HELLO: &str = "Hello, world!";
    println!("name is: {}", HELLO);

    static mut COUNTER: u32 = 0;
    unsafe {
        COUNTER += 1;
        println!("COUNTER: {}", COUNTER);
    }
}

#[test]
fn implement_unsafe_trait() {
    // 六. 实现不安全 trait
    unsafe trait Foo {
        // methods go here
    }

    unsafe impl Foo for i32 {
        // method implementations go here
    }
}

fn main() {
    // 1. 不安全的超能力
    // 可以通过 unsafe 关键字来切换到不安全 Rust，接着可以开启一个新的存放不安全代码的块。
    // 这里有五类可以在不安全 Rust 中进行而不能用于安全 Rust 的操作，它们称之为 “不安全的超能力。” 这些超能力是：

    // 解引用裸指针
    // 调用不安全的函数或方法
    // 访问或修改可变静态变量
    // 实现不安全 trait
    // 访问 union 的字段
}
