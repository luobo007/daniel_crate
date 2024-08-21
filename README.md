daniel_crate 是一个用于数学计算的Rust库，提供了一些基本的数学操作，例如加法函数。该库设计简单，易于集成到其他Rust项目中。

项目简介
daniel_crate 提供了简单的数学计算工具，目前支持以下功能：

add 函数：将两个 u8 类型的数字相加，并在结果超出范围时触发 panic。
安装指南
要在您的项目中使用 daniel_crate，首先需要在 Cargo.toml 文件中添加依赖项：
[dependencies]
daniel_crate = "0.1.0"

使用许可证
daniel_crate 使用 Apache-2.0 许可证。更多信息请参阅 LICENSE 文件。

然后，您可以在项目中使用该库：

使用示例
以下是如何在项目中使用 daniel_crate src/lib.rs的简单示例：
fn it_works() {
        let result = add(1, 1);
        assert_eq!(result, 2); // 检查1 + 1的结果是否为2
}

贡献指南
如果您想为 daniel_crate 做出贡献，请遵循以下步骤：
Fork 仓库并克隆到您的本地环境。
创建一个新分支并在该分支上进行开发。
提交更改并推送到您的分支。
创建一个 Pull Request，描述您的更改内容。
欢迎任何形式的贡献，无论是修复bug、添加新功能，还是改善文档。

