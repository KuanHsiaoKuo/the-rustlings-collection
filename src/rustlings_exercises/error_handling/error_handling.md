# Error handling

Most errors aren’t serious enough to require the program to stop entirely.
Sometimes, when a function fails, it’s for a reason that you can easily interpret and respond to.
For example, if you try to open a file and that operation fails because the file doesn’t exist, you might want to create the file instead of terminating the process.

## Further information

- [Error Handling](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
- [Generics](https://doc.rust-lang.org/book/ch10-01-syntax.html)
- [Result](https://doc.rust-lang.org/rust-by-example/error/result.html)
- [Boxing errors](https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/boxing_errors.html)

## Rustlings

~~~admonish note title="errors1" collapsible=true
```rust
{{#include errors1.rs}}
```
~~~

~~~admonish note title="errors2" collapsible=true
```rust
{{#include errors2.rs}}
```
~~~

~~~admonish note title="errors3" collapsible=true
```rust
{{#include errors3.rs}}
```
~~~

~~~admonish note title="errors4" collapsible=true
```rust
{{#include errors4.rs}}
```
~~~

~~~admonish note title="errors5" collapsible=true
```rust
{{#include errors5.rs}}
```
~~~

~~~admonish note title="errors6" collapsible=true
```rust
{{#include errors6.rs}}
```
~~~
