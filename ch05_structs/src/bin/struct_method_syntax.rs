#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// 为了使函数定义于 Rectangle 的上下文中，我们开始了一个 impl 块
// （impl 是 implementation 的缩写），这个 impl 块中的所有内容都将与 Rectangle 类型相关联。
impl Rectangle {
    // 使用 &self 来替代 rectangle: &Rectangle，&self 实际上是 self: &Self 的缩写
    // 方法的第一个参数必须有一个名为 self 的Self 类型的参数，所以 Rust 让你在第一个参数位置上只用 self 这个名字来缩写。
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 多个参数
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // 所有在 impl 块中定义的函数被称为关联函数（associated function），因为它们与 impl 后面命名的类型相关。
    // 我们可以定义不以 self 为第一参数的关联函数（因此不是方法），因为它们并不作用于一个结构体的实例。
    // Rectangle::square(30);   使用::调用关联函数
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    println!("rect1's area is {}", rect1.area());
    println!("can rect1 hold rect2 {}", rect2.can_hold(&rect1));
    let square = Rectangle::square(30);
    println!("{:?}", square);
}