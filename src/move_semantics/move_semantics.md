# Move Semantics

<!--ts-->
* [Move Semantics](#move-semantics)
   * [Further information](#further-information)
   * [Rustlings](#rustlings)
      * [借用](#借用)
      * [可变借用](#可变借用)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Sun Mar  5 08:47:30 UTC 2023 -->

<!--te-->
These exercises are adapted from [pnkfelix](https://github.com/pnkfelix)'s [Rust Tutorial](https://pnkfelix.github.io/rust-examples-icfp2014/) -- Thank you Felix!!!

## Further information

For this section, the book links are especially important.

- [Ownership](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
- [Reference and borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)

## Rustlings

### 借用

~~~admonish note title="move_semantics1" collapsible=true
```rust,editable
{{#include move_semantics1.rs}}
```
~~~

~~~admonish tip title="Hint" collapsible=true
So you've got the "cannot borrow immutable local variable `vec1` as mutable" error on line 13,
right? The fix for this is going to be adding one keyword, and the addition is NOT on line 13
where the error is.

Also: Try accessing `vec0` after having called `fill_vec()`. See what happens!
~~~

### 可变借用

~~~admonish note title="move_semantics2" collapsible=true
```rust,editable
{{#include move_semantics2.rs}}
```
~~~

~~~admonish tip title="Hint" collapsible=true
So, `vec0` is passed into the `fill_vec` function as an argument. In Rust,
when an argument is passed to a function and it's not explicitly returned,
you can't use the original variable anymore. We call this "moving" a variable.
Variables that are moved into a function (or block scope) and aren't explicitly
returned get "dropped" at the end of that function. This is also what happens here.
There's a few ways to fix this, try them all if you want:
1. Make another, separate version of the data that's in `vec0` and pass that
   to `fill_vec` instead.
2. Make `fill_vec` borrow its argument instead of taking ownership of it,
   and then copy the data within the function in order to return an owned
   `Vec<i32>`
3. Make `fill_vec` *mutably* borrow a reference to its argument (which will need to be
   mutable), modify it directly, then not return anything. Then you can get rid
   of `vec1` entirely -- note that this will change what gets printed by the
   first `println!`
~~~

~~~admonish note title="move_semantics3" collapsible=true
```rust,editable
{{#include move_semantics3.rs}}
```
~~~

~~~admonish tip title="Hint" collapsible=true
The difference between this one and the previous ones is that the first line
of `fn fill_vec` that had `let mut vec = vec;` is no longer there. You can,
instead of adding that line back, add `mut` in one place that will change
an existing binding to be a mutable binding instead of an immutable one :)
~~~

~~~admonish note title="move_semantics4" collapsible=true
```rust,editable
{{#include move_semantics4.rs}}
```
~~~

~~~admonish tip title="Hint" collapsible=true
Stop reading whenever you feel like you have enough direction :) Or try
doing one step and then fixing the compiler errors that result!
So the end goal is to:
   - get rid of the first line in main that creates the new vector
   - so then `vec0` doesn't exist, so we can't pass it to `fill_vec`
   - we don't want to pass anything to `fill_vec`, so its signature should
     reflect that it does not take any arguments
   - since we're not creating a new vec in `main` anymore, we need to create
     a new vec in `fill_vec`, similarly to the way we did in `main`
~~~

~~~admonish note title="move_semantics5" collapsible=true
```rust,editable
{{#include move_semantics5.rs}}
```
~~~

~~~admonish tip title="Hint" collapsible=true
Carefully reason about the range in which each mutable reference is in
scope. Does it help to update the value of referent (x) immediately after
the mutable reference is taken? Read more about 'Mutable References':
> [References and Borrowing > Mutable References - The Rust Programming Language](https://kuanhsiaokuo.github.io/the-rust-programming-book-khk/ch04-02-references-and-borrowing.html#mutable-references)
~~~

~~~admonish note title="move_semantics6" collapsible=true
```rust,editable
{{#include move_semantics6.rs}}
```
~~~

~~~admonish tip title="Hint" collapsible=true
To find the answer, you can consult the book section ["References and Borrowing"](https://kuanhsiaokuo.github.io/the-rust-programming-book-khk/ch04-02-references-and-borrowing.html#references-and-borrowing)

1. The first problem is that `get_char` is taking ownership of the string.
So `data` is moved and can't be used for `string_uppercase`
`data` is moved to `get_char` first, meaning that `string_uppercase` cannot manipulate the data.
Once you've fixed that, `string_uppercase`'s function signature will also need to be adjusted.
Can you figure out how?

2. Another hint: it has to do with the `&` character.
~~~
