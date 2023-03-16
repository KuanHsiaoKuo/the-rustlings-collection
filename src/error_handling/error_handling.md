# Error handling

<!--ts-->
* [Error handling](#error-handling)
   * [Is Option part of error handling](#is-option-part-of-error-handling)
   * [Further information](#further-information)
   * [Rustlings](#rustlings)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Thu Mar 16 14:35:08 UTC 2023 -->

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
    - [Unpacking options with ? to return the Some or terminate - The Rust Programming Language](https://kuanhsiaokuo.github.io/the-rust-programming-book-khk/rust_by_example_src/error/option_unwrap/question_mark.html)
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
- [Unpacking options with ? to return the Some or terminate - The Rust Programming Language](https://kuanhsiaokuo.github.io/the-rust-programming-book-khk/rust_by_example_src/error/option_unwrap/question_mark.html)
- [Other uses of ? - The Rust Programming Language](https://kuanhsiaokuo.github.io/the-rust-programming-book-khk/rust_by_example_src/error/multiple_error_types/reenter_question_mark.html)

~~~

~~~admonish danger title="⚡️️solution: just like async fn to accept await -> return Result<T, E> to chain ?" collapsible=true
```rust, editable
fn main() -> Result<(), ParseIntError>{ // main function also could return Result<T, E>
    let mut tokens = 100;
    let pretend_user_input = "8";

    let cost = total_cost(pretend_user_input)?; // should return Result to accept ?, if got error, here will return ParseIntError

    if cost > tokens {
        println!("You can't afford that many!");
    } else {
        tokens -= cost;
        println!("You now have {} tokens.", tokens);
    }
    Ok(()) // if successed, here return the OK result ()
}
```
~~~

---

~~~admonish note title="errors4" collapsible=true
```rust,editable
{{#include errors4.rs}}
```
~~~

~~~admonish tip title="Hint" collapsible=true
> `PositiveNonzeroInteger::new` is always creating a new instance and returning an `Ok` result.

It should be doing some checking, returning an `Err` result if those checks fail, and only
returning an `Ok` result if those checks determine that everything is... okay :)
~~~

~~~admonish success title="solution1: use if to catch err" collapsible=true
```rust, editable
impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        // Hmm...? Why is this only returning an Ok value?
        if value < 0 {
            Err(CreationError::Negative)
        } else if value == 0 {
            Err(CreationError::Zero)
        } else {
            Ok(PositiveNonzeroInteger(value as u64))
        }      
    }
}
```
~~~

~~~admonish success title="solution2: use match to catch err" collapsible=true
- [Guards: to filter the arm - The Rust Programming Language](https://kuanhsiaokuo.github.io/the-rust-programming-book-khk/rust_by_example_src/flow_control/match/guard.html)
- [@ Binding - The Rust Programming Language](https://kuanhsiaokuo.github.io/the-rust-programming-book-khk/rust_by_example_src/flow_control/match/binding.html)
```rust, editable
impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        // Hmm...? Why is this only returning an Ok value?
        match value {
            v if v < 0 => return Err(CreationError::Negative),
            v if v == 0 => return Err(CreationError::Zero),
            _ => return Ok(PositiveNonzeroInteger(value as u64))
        }
    }
}
```
~~~

---

- This exercise uses some concepts that we won't get to until later in the course, like `Box` and the
  `From` trait.
- It's not important to understand them in detail right now, but you can read ahead if you like.

> For now, think of the `Box<dyn ...>` type as an "I want anything that does ???" type, which, given
> Rust's usual standards for runtime safety, should strike you as somewhat lenient!

- In short, this particular use case for boxes is for when you want to own a value and you care only that it is a type which implements a particular trait.
- To do so, The Box is declared as of type `Box<dyn Trait>` where Trait is the trait the compiler looks for on any value used in that context.
- For this exercise, that context is the potential errors which can be returned in a Result.

> What can we use to describe both errors? In other words, is there a trait which both errors implement?

~~~admonish note title="errors5" collapsible=true
```rust,editable
{{#include errors5.rs}}
```
~~~

~~~admonish tip title="Hint" collapsible=true
There are two different possible `Result` types produced within `main()`, which are
propagated using `?` operators. 

> How do we declare a return type from `main()` that allows both?

- Under the hood, the `?` operator calls `From::from` on the error value to convert it to a boxed trait object, a `Box<dyn error::Error>`. 
- This boxed trait object is `polymorphic`, and since all errors implement the `error::Error` trait, we can capture lots of different errors in one "Box" object.

Check out this section of the book:
https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#a-shortcut-for-propagating-errors-the--operator

Read more about boxing errors: [⭐️Recoverable Errors with Result - The Rust Programming Language](https://kuanhsiaokuo.github.io/the-rust-programming-book-khk/ch09-02-recoverable-errors-with-result.html#the--operator-a-shortcut-for-propagating-errors)

Read more about using the `?` operator with boxed errors: [Other uses of ? - The Rust Programming Language](https://kuanhsiaokuo.github.io/the-rust-programming-book-khk/rust_by_example_src/error/multiple_error_types/reenter_question_mark.html)
~~~

~~~admonish success title="solution: error::Error trait" collapsible=true
```rust, editable
Box<dyn error::Error>
```
~~~

---
Using catch-all error types like `Box<dyn error::Error>` isn't recommended
for library code:
> where callers might want to make decisions based on the
> error content, instead of printing it out or propagating it further.

Here, we define a custom error type to make it possible for callers to decide
what to do next when our function returns an error.

~~~admonish note title="errors6" collapsible=true
```rust,editable
{{#include errors6.rs}}
```
~~~

~~~admonish tip title="Hint" collapsible=true
> This exercise uses a completed version of `PositiveNonzeroInteger` from
> errors4.

- Below the line that TODO asks you to change, there is an example of using the `map_err()` method on a `Result` to transform one type of error into another.
- Try using something similar on the `Result` from `parse()`.
- You might use the `?` operator to return early from the function
- or you might use a `match` expression, or maybe there's another way!

> You can create another function inside `impl ParsePosNonzeroError` to use
> with `map_err()`.

Read more about `map_err()` in the `std::result` documentation:
[Result in std::result - Rust](https://doc.rust-lang.org/std/result/enum.Result.html#method.map_err)
~~~

~~~admonish success title="solution: define a custom error type" collapsible=true
```rust, editable
impl ParsePosNonzeroError {
    fn from_creation(err: CreationError) -> ParsePosNonzeroError {
        ParsePosNonzeroError::Creation(err)
    }
    // TODO: add another error conversion function here.
    // fn from_parseint...
    fn from_parseint(err: ParseIntError) -> ParsePosNonzeroError {
        ParsePosNonzeroError::ParseInt(err)
    }
}

fn parse_pos_nonzero(s: &str)
    -> Result<PositiveNonzeroInteger, ParsePosNonzeroError>
{
    // TODO: change this to return an appropriate error instead of panicking
    // when `parse()` returns an error.
    // let x: i64 = s.parse().unwrap();
    let x: i64 = s.parse().map_err(ParsePosNonzeroError::from_parseint)?;

    PositiveNonzeroInteger::new(x)
        .map_err(ParsePosNonzeroError::from_creation)
}
```
~~~

