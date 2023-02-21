# Lifetimes

<!--ts-->

* [Lifetimes](#lifetimes)
    * [Further information](#further-information)
    * [Rustlings](#rustlings)
        * [expected named lifetime parameter](#expected-named-lifetime-parameter)
        * [borrowed value does not live long enough](#borrowed-value-does-not-live-long-enough)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Tue Feb 21 11:03:29 UTC 2023 -->

<!--te-->
Lifetimes tell the compiler how to check:

> whether `references` live long enough to be valid in any given situation.

For example lifetimes say "make sure parameter 'a' lives as long as parameter 'b' so that the return value is valid".

> They are only necessary on borrows, i.e. references

- Since copied parameters or moves are owned in their scope and cannot
  be referenced outside.
- Lifetimes mean that calling code of e.g. functions
  can be checked to make sure their arguments are valid.
- Lifetimes are restrictive of their callers.

## Further information

- [Validating References with Lifetimes](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)
- [Lifetimes (in Rust By Example)](https://doc.rust-lang.org/stable/rust-by-example/scope/lifetime.html)
- [Lifetimes - The Rust Programming Language](https://kuanhsiaokuo.github.io/the-rust-programming-book-khk/rust_by_example_src/scope/lifetime.html)

## Rustlings

### expected named lifetime parameter

~~~admonish note title="function" collapsible=true
```rust,editable
{{#include lifetimes1.rs}}
```
~~~

~~~admonish note title="struct" collapsible=true
```rust,editable
{{#include lifetimes3.rs}}
```
~~~

### borrowed value does not live long enough

~~~admonish note title="lifetimes2" collapsible=true
```rust,editable
{{#include lifetimes2.rs}}
```
~~~

