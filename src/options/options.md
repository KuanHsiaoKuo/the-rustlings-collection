# Options

Type Option represents an optional value: every Option is either Some and contains a value, or None, and does not.
Option types are very common in Rust code, as they have a number of uses:

- Initial values
- Return values for functions that are not defined over their entire input range (partial functions)
- Return value for otherwise reporting simple errors, where None is returned on error
- Optional struct fields
- Struct fields that can be loaned or "taken"
- Optional function arguments
- Nullable pointers
- Swapping things out of difficult situations

## Further Information

- [✨Generic Data Types->Option Enum Format - The Rust Programming Language](https://kuanhsiaokuo.github.io/the-rust-programming-book-khk/ch10-01-syntax.html#in-enum-definitions)
- [Option Module Documentation](https://doc.rust-lang.org/std/option/)
- [Option Enum Documentation](https://doc.rust-lang.org/std/option/enum.Option.html)
- [if let - The Rust Programming Language](https://kuanhsiaokuo.github.io/the-rust-programming-book-khk/rust_by_example_src/flow_control/if_let.html)
- [while let - The Rust Programming Language](https://kuanhsiaokuo.github.io/the-rust-programming-book-khk/rust_by_example_src/flow_control/while_let.html)

## Rustlings

~~~admonish note title="options1" collapsible=true
```rust,editable
{{#include options1.rs}}
```
~~~

~~~admonish note title="options2" collapsible=true
```rust,editable
{{#include options2.rs}}
```
~~~

~~~admonish note title="options3" collapsible=true
```rust,editable
{{#include options3.rs}}
```
~~~
