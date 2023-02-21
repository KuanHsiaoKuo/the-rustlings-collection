# Lifetimes

<!--ts-->
* [Lifetimes](#lifetimes)
   * [Further information](#further-information)
   * [Rustlings](#rustlings)
      * [expected named lifetime parameter](#expected-named-lifetime-parameter)
      * [borrowed value does not live long enough](#borrowed-value-does-not-live-long-enough)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Tue Feb 21 11:09:28 UTC 2023 -->

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

~~~admonish bug title="function" collapsible=true
```rust,editable
{{#include lifetimes1.rs}}
```
~~~

~~~admonish tip title="Hint" collasible=true
Let the compiler guide you. Also take a look at [the book](https://kuanhsiaokuo.github.io/the-rust-programming-book-khk/ch10-03-lifetime-syntax.html) if you need help:
~~~

~~~admonish bug title="struct" collapsible=true
```rust,editable
{{#include lifetimes3.rs}}
```
~~~

### borrowed value does not live long enough

~~~admonish bug title="lifetimes2" collapsible=true
```rust,editable
{{#include lifetimes2.rs}}:
```
~~~

~~~admonish tip title="Hint" collasible=true
If you use a lifetime annotation in a struct's fields, where else does it need to be added?
~~~

~~~admonish tip title="Hint" collasible=true
Remember that the generic lifetime 'a will get the concrete lifetime that is equal to the smaller of the lifetimes of x and y.
You can take at least two paths to achieve the desired result while keeping the inner block:
1. Move the string2 declaration to make it live as long as string1 (how is result declared?)
2. Move println! into the inner block
~~~

~~~admonish info title="Q&A" collsible=true
## Q: 为什么不能改成
```rust
fn longest<'a:'b, 'b>(x: &'a str, y: &'b str) -> &'b str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```
> A: 因为可能返回'a x, 也可能返回'b y. 这里主要问题在于返回的引用有两种生命周期。
~~~