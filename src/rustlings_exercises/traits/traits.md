# Traits

A trait is a collection of methods.

Data types can implement traits. To do so, the methods making up the trait are defined for the data type. For example, the `String` data type implements the `From<&str>` trait. This allows a user to write `String::from("hello")`.

In this way, traits are somewhat similar to Java interfaces and C++ abstract classes.

Some additional common Rust traits include:

- `Clone` (the `clone` method)
- `Display` (which allows formatted display via `{}`)
- `Debug` (which allows formatted display via `{:?}`)

Because traits indicate shared behavior between data types, they are useful when writing generics.

## Further information

- [Traits](https://doc.rust-lang.org/book/ch10-02-traits.html)

## Rustlings

~~~admonish note title="traits1" collapsible=true
```rust,editable
{{#include traits1.rs}}
```
~~~

~~~admonish note title="traits2" collapsible=true
```rust,editable
{{#include traits2.rs}}
```
~~~

~~~admonish note title="traits3" collapsible=true
```rust,editable
{{#include traits3.rs}}
```
~~~

~~~admonish note title="traits4" collapsible=true
```rust,editable
{{#include traits4.rs}}
```
~~~

~~~admonish note title="traits5" collapsible=true
```rust,editable
{{#include traits5.rs}}
```
~~~
