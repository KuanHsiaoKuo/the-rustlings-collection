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

~~~admonish tip title="Hint" collapsible=true
A discussion about Traits in Rust can be found at:
[🪐Traits: Defining Shared Behavior - The Rust Programming Language](https://kuanhsiaokuo.github.io/the-rust-programming-book-khk/ch10-02-traits.html)
~~~

---

~~~admonish note title="traits2" collapsible=true
```rust,editable
{{#include traits2.rs}}
```
~~~

~~~admonish tip title="Hint" collapsible=true
Notice how the trait takes ownership of 'self',and returns `Self`.
Try mutating the incoming string vector. Have a look at the tests to see
what the result should look like!

Vectors provide suitable methods for adding an element at the end. See
the documentation at: 
[Vec in std::vec - Rust](https://doc.rust-lang.org/std/vec/struct.Vec.html)
~~~

---

~~~admonish note title="traits3" collapsible=true
```rust,editable
{{#include traits3.rs}}
```
~~~

~~~admonish tip title="Hint" collapsible=true
Traits can have a default implementation for functions. Structs that implement
the trait can then use the default version of these functions if they choose not
implement the function themselves.

See the documentation at: 
[🪐Traits: Defining Shared Behavior >> Default Implementations - The Rust Programming Language](https://kuanhsiaokuo.github.io/the-rust-programming-book-khk/ch10-02-traits.html#default-implementations)
~~~

---

~~~admonish note title="traits4" collapsible=true
```rust,editable
{{#include traits4.rs}}
```
~~~

~~~admonish tip title="Hint" collapsible=true
Instead of using concrete types as parameters you can use traits. Try replacing the
'??' with 'impl <what goes here?>'

See the documentation at:
[🪐Traits: Defining Shared Behavior >> Traits as Parameters - The Rust Programming Language](https://kuanhsiaokuo.github.io/the-rust-programming-book-khk/ch10-02-traits.html#traits-as-parameters)
~~~

---

~~~admonish note title="traits5" collapsible=true
```rust,editable
{{#include traits5.rs}}
```
~~~

~~~admonish tip title="Hint" collapsible=true
To ensure a parameter implements multiple traits use the '+ syntax'. Try replacing the
'??' with 'impl <> + <>'.

See the documentation at:
[Specifying Multiple Trait Bounds with the + Syntax](https://kuanhsiaokuo.github.io/the-rust-programming-book-khk/ch10-02-traits.html#specifying-multiple-trait-bounds-with-the--syntax)
~~~
