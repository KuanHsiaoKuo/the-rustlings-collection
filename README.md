# Rust错题本

## 缘起

> 灵感来自：[rust-lang/rustlings: Small exercises to get you used to reading and writing Rust code!](https://github.com/rust-lang/rustlings)

余学Rust一载有余，深感其"一说就会，一写就废"的特点。在整理了几本电子书后，受到rustlings启发，觉得确实可以起一本"Rust错题本".

## rustlings不错，更需要自己整理

rustlings使用起来确实很方便，其代码也值得研究，是常见的rust入门demo类型-命令行工具。但是它必须在电脑上运行，不适合随时练习，所以觉得mdbook在线运行会更方便。

## admonish-mdbook

[Reference - The mdbook-admonish book](https://tommilligan.github.io/mdbook-admonish/reference.html)

## 单元测试与main函数

rustlings默认代码中大都使用单元测试进行验证，这也和其本身用意相符：通过编译器的测试。但是我都手动转为main函数，这是为了方便在网页上可以在rust playground中执行。