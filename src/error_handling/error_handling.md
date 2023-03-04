# Error handling

<!--ts-->
* [Error handling](#error-handling)
   * [Is Option part of error handling](#is-option-part-of-error-handling)
   * [Further information](#further-information)
   * [Rustlings](#rustlings)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Sat Mar  4 12:09:37 UTC 2023 -->

<!--te-->

1. Most errors aren’t serious enough to require the program to stop entirely.
2. Sometimes, when a function fails, it’s for a reason that you can easily interpret and respond to.

> For example, if you try to open a file and that operation fails because the file doesn’t exist, you might want to create the file instead of terminating the process.

## Is Option<T> part of error handling

> In Rust, an "option" is not technically part of error handling, but it is often used in combination with error handling to represent the possibility of a value being absent or "None."

1. An "option" is a type that represents either Some value or None.

- When a function may not return a value, it can return an Option<T> where T is the type of the value that might be returned.
- If the function succeeds and returns a value, it returns Some(value); otherwise, it returns None.

> Error handling in Rust typically uses the Result<T, E> type

- where T is the type of the value that is returned if the operation succeeds
- and E is the type of the error that may occur.
- When an error occurs, a value of type E is returned, and when the operation succeeds, a value of type T is returned.

> So while Option and Result are different types in Rust, they are often used in combination to handle situations where a value may or may not be present or when an operation may or may not succeed.

## Further information

- [Rust 使用 Result 的错误处理方式与 Golang 使用 error 的方式有什么本质区别？ - 知乎](https://www.zhihu.com/question/36444352): [dt link](x-devonthink-item://21BCF1DC-17A0-428C-BA20-A1F1A515B207)
- [错误处理内容和主流方法 - Anatomy In First Rust Programming Class 🦀](https://kuanhsiaokuo.github.io/geektime-tyr-rust/3_3_1_error_content.html)
- [⭐️Recoverable Errors with Result - The Rust Programming Language](https://kuanhsiaokuo.github.io/the-rust-programming-book-khk/ch09-02-recoverable-errors-with-result.html)
- [✨Generic Data Types - The Rust Programming Language](https://kuanhsiaokuo.github.io/the-rust-programming-book-khk/ch10-01-syntax.html)
- [⭐️Error handling: Panic、Option and Result - The Rust Programming Language](https://kuanhsiaokuo.github.io/the-rust-programming-book-khk/rust_by_example_src/error.html)
    - [Result & OK or ?: A Richer Version Of Option - The Rust Programming Language](https://kuanhsiaokuo.github.io/the-rust-programming-book-khk/rust_by_example_src/error/result.html)
    - [Boxing errors - The Rust Programming Language](https://kuanhsiaokuo.github.io/the-rust-programming-book-khk/rust_by_example_src/error/multiple_error_types/boxing_errors.html)

> [rustlings-solutions-5/error_handling at main · gaveen/rustlings-solutions-5](https://github.com/gaveen/rustlings-solutions-5/tree/main/error_handling)

## Rustlings

~~~admonish note title="errors1: change Option<T> to Result<T, E>" collapsible=true
```rust,editable
{{#include errors1.rs}}
```
~~~

~~~admonish tip title="Hint" collapsible=true
`Ok` and `Err` are one of the variants of `Result`, so what the tests are saying
is that `generate_nametag_text` should return a `Result` instead of an
`Option`.

To make this change, you'll need to:
   - update the return type in the function signature to be a Result<String, String> that
     could be the variants `Ok(String)` and `Err(String)`
   - change the body of the function to return `Ok(stuff)` where it currently
     returns `Some(stuff)`
   - change the body of the function to return `Err(error message)` where it
     currently returns `None`
~~~

~~~admonish success title="solution: Result<String, String>" collapsible=true
```rust,editable
pub fn generate_nametag_text(name: String) -> Result<String, String> {
    if name.is_empty() {
        // Empty names aren't allowed.
        Err(format!("`name` was empty; it must be nonempty."))
    } else {
        Ok(format!("Hi! My name is {}", name))
    }
}
```
1. 这里其实单元测试代码已经指出要报错的内容
2. Result<T, E>中，E其实就是各种Err
~~~

---

- Say we're writing a game where you can buy items with tokens. All items cost
  5 tokens, and whenever you purchase items there is a processing fee of 1
  token.
- A player of the game will type in how many items they want to buy,
  and the `total_cost` function will calculate the total number of tokens.
- Since the player typed in the quantity, though, we get it as a string-- and
  they might have typed anything, not just numbers!

> Right now, this function isn't handling the error case at all (and isn't
> handling the success case properly either). What we want to do is:

- if we call the `parse` function on a string that is not a number, that
  function will return a `ParseIntError`,
- and in that case, we want to
  immediately return that error from our function and not try to multiply
  and add.

> There are at least two ways to implement this that are both correct-- but
> one is a lot shorter!

~~~admonish note title="⭐️errors2: unwrap_err()" collapsible=true
```rust,editable
{{#include errors2.rs}}
```
~~~

~~~admonish tip title="Hint" collapsible=true
1. One way to handle this is using a `match` statement on
   `item_quantity.parse::<i32>()` where the cases are `Ok(something)` and
   `Err(something)`. This pattern is very common in Rust, though, so there's
   a `?` operator that does pretty much what you would make that match statement
   do for you!
2. Take a look at this section of the `match expression` part in Error Handling chapter:
- [⭐️Recoverable Errors with Result - The Rust Programming Language](https://kuanhsiaokuo.github.io/the-rust-programming-book-khk/ch09-02-recoverable-errors-with-result.html#match-expression-how-to-hanle-the-information)
- [Early returns: Catch Exception and Return - The Rust Programming Language](https://kuanhsiaokuo.github.io/the-rust-programming-book-khk/rust_by_example_src/error/result/early_returns.html)
   and give it a try!
~~~

~~~admonish success title="solution1: match err to panic! return" collapsible=true
```rust, editable
pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = match item_quantity.parse::<i32>(){
        Ok(iqty) => iqty,
        Err(error) => panic!("Problem parsing the item_quantity: {:?}", error),
    };

    Ok(qty * cost_per_item + processing_fee)
}
```
~~~

~~~admonish success title="solution2: match err to early return" collapsible=true
```rust, editable
pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = match item_quantity.parse::<i32>(){
        Ok(iqty) => iqty,
        Err(e) => return Err(e),
    };

    Ok(qty * cost_per_item + processing_fee)
}
```
~~~

~~~admonish danger title="⌛️solution3: return custom error message" collapsible=true
```rust, editable
todo()
// return Err(String::from("invalid digit found in string")),
```
~~~

---

~~~admonish note title="errors3: ? -> return *Result* or *Option* to accept *?*" collapsible=true
```rust,editable
{{#include errors3.rs}}
```
~~~

~~~admonish tip title="Hint" collapsible=true
> If other functions can return a `Result`, why shouldn't `main`? 

It's a fairly common
convention to return something like Result<(), ErrorType> from your main function.
The unit (`()`) type is there because nothing is really needed in terms of positive
results.
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
