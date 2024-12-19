// cfg 属性代表 configuration
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

fn main() {
    // 1. 单元测试
    // 单元测试的目的是在与其他部分隔离的环境中测试每一个单元的代码，以便于快速而准确的某个单元的代码功能是否符合预期。
    // 单元测试与他们要测试的代码共同存放在位于 src 目录下相同的文件中。
    // 规范是在每个文件中创建包含测试函数的 tests 模块，并使用 cfg(test) 标注模块。

    // 2. 集成测试
    // 2.1. 新建tests目录 和src同级
    // Cargo 知道如何去寻找这个目录中的集成测试文件。接着可以随意在这个目录中创建任意多的测试文件，
    // Cargo 会将每一个文件当作单独的 crate 来编译。 这句话划重点。

    // 详情查看
    // https://www.rustwiki.org.cn/zh-CN/book/ch11-03-test-organization.html
}