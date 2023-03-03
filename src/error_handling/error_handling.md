# Error handling

1. Most errors aren’t serious enough to require the program to stop entirely.
2. Sometimes, when a function fails, it’s for a reason that you can easily interpret and respond to.

> For example, if you try to open a file and that operation fails because the file doesn’t exist, you might want to create the file instead of terminating the process.

## Further information

- [错误处理内容和主流方法 - Anatomy In First Rust Programming Class 🦀](https://kuanhsiaokuo.github.io/geektime-tyr-rust/3_3_1_error_content.html)
- [⭐️Recoverable Errors with Result - The Rust Programming Language](https://kuanhsiaokuo.github.io/the-rust-programming-book-khk/ch09-02-recoverable-errors-with-result.html)
- [✨Generic Data Types - The Rust Programming Language](https://kuanhsiaokuo.github.io/the-rust-programming-book-khk/ch10-01-syntax.html)
- [⭐️Error handling: Panic、Option and Result - The Rust Programming Language](https://kuanhsiaokuo.github.io/the-rust-programming-book-khk/rust_by_example_src/error.html)
    - [Result & OK or ?: A Richer Version Of Option - The Rust Programming Language](https://kuanhsiaokuo.github.io/the-rust-programming-book-khk/rust_by_example_src/error/result.html)
    - [Boxing errors - The Rust Programming Language](https://kuanhsiaokuo.github.io/the-rust-programming-book-khk/rust_by_example_src/error/multiple_error_types/boxing_errors.html)

## Rustlings

~~~admonish note title="errors1" collapsible=true
```rust,editable
{{#include errors1.rs}}
```
~~~

~~~admonish note title="errors2" collapsible=true
```rust,editable
{{#include errors2.rs}}
```
~~~

~~~admonish note title="errors3" collapsible=true
```rust,editable
{{#include errors3.rs}}
```
~~~

~~~admonish note title="errors4" collapsible=true
```rust,editable
{{#include errors4.rs}}
```
~~~

~~~admonish note title="errors5" collapsible=true
```rust,editable
{{#include errors5.rs}}
```
~~~

~~~admonish note title="errors6" collapsible=true
```rust,editable
{{#include errors6.rs}}
```
~~~
