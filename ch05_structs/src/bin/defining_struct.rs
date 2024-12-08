// 1. 普通结构体
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// 2. 元组结构体（tuple struct）
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// 3. 没有任何字段的 类单元结构体（unit-like structs）
struct AlwaysEqual;
fn main() {
    // 更多查看 https://www.rustwiki.org.cn/zh-CN/book/ch05-01-defining-structs.html
    let user = User {
        active: true,
        username: String::from("seven"),
        email: String::from("example@mail.com"),
        sign_in_count: 1,
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;


    let user1 = User {
        // 根据定义字段的数据类型来决定是拷贝还是移动 这里用到了 所有权 的知识点
        active: user.active,   // 拷贝Copy
        username: user.username, // 移动 Move
        email: String::from("another@example.com"),
        sign_in_count: user.sign_in_count, // 拷贝Copy
    };
    println!("user: {}", user.email);
    // println!("username: {}", user.username); // 报错


    // 结构体更新语法
    // 和上面一样 但是更简洁
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    println!("user1: {}", user1.email);
}

fn build_user(username: String, email: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}