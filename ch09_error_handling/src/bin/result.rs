use std::fs::File;
use std::io;
use std::io::Read;

fn main() {
    // 1.
    // let f = File::open("hello.txt");
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         other_error => panic!("Problem opening the file: {:?}", other_error),
    //     },
    // };

    // 这段代码和上面一样 后面来学...
    let _f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == io::ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });


    // 2. 失败时 panic 的简写：unwrap 和 expect
    // match 能够胜任它的工作，不过它可能有点冗长并且不总是能很好的表明其意图。Result<T, E> 类型定义了很多辅助方法来处理各种情况。
    // 其中之一叫做 unwrap，它的实现就类似于示例 9-4 中的 match 语句。如果 Result 值是成员 Ok，unwrap 会返回 Ok 中的值。
    // 如果 Result 是成员 Err，unwrap 会为我们调用 panic!
    // let f = File::open("hello.txt").unwrap();

    // 还有另一个类似于 unwrap 的方法它还允许我们选择 panic! 的错误信息：expect。
    // 使用 expect 而不是 unwrap 并提供一个好的错误信息可以表明你的意图并更易于追踪 panic 的根源
    // let f = File::open("hello.txt").expect("Failed to open hello.txt");


    // 3. 传递错误
    match read_username_from_file() {
        Ok(name) => println!("name: {}", name),
        Err(err) => panic!("read_username_from_file error: {:?}", err),
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    // let f = File::open("hello.txt");
    // let mut s = String::new();
    //
    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };
    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e),
    // }

    // 传播错误的简写：? 运算符
    // 这串代码这和上面一样
    // let mut f = File::open("hello.txt")?;
    // let mut s = String::new();
    // f.read_to_string(&mut s)?;
    // Ok(s);
    // Result 值之后的 ? 被定义为与示例 9-6 中定义的处理 Result 值的 match 表达式有着完全相同的工作方式。
    // 如果 Result 的值是 Ok，这个表达式将会返回 Ok 中的值而程序将继续执行。
    // 如果值是 Err，Err 中的值将作为整个函数的返回值，就好像使用了 return 关键字一样，这样错误值就被传播给了调用者

    // match 表达式与问号运算符所做的有一点不同：? 运算符所使用的错误值被传递给了 from 函数，它定义于标准库的 From trait 中，
    // 其用来将错误从一种类型转换为另一种类型。当 ? 运算符调用 from 函数时，收到的错误类型被转换为由当前函数返回类型所指定的错误类型
    // 。这在当函数返回单个错误类型来代表所有可能失败的方式时很有用，即使其可能会因很多种原因失败。
    // 只要每一个错误类型都实现了 from 函数来定义如何将自身转换为返回的错误类型，? 运算符会自动处理这些转换。

    // ? 运算符消除了大量样板代码并使得函数的实现更简单。我们甚至可以在 ? 之后直接使用链式方法调用来进一步缩短代码。
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}