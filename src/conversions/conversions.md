# Type conversions

<!--ts-->
* [Type conversions](#type-conversions)
   * [Further information](#further-information)
   * [Rustlings](#rustlings)
      * [as_ref_mut](#as_ref_mut)
      * [from_into](#from_into)
      * [from_str](#from_str)
      * [try_from_into](#try_from_into)
      * [using_as](#using_as)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Sat Feb 25 13:13:52 UTC 2023 -->

<!--te-->
Rust offers a multitude of ways to convert a value of a given type into another type.

The simplest form of type conversion is a type cast expression. It is denoted with the binary operator `as`. For instance, `println!("{}", 1 + 1.0);` would not compile, since `1` is an integer while `1.0` is a float. However, `println!("{}", 1 as f32 + 1.0)` should compile. The
exercise [`using_as`](using_as.rs) tries to cover this.

Rust also offers traits that facilitate type conversions upon implementation. These traits can be found under the [`convert`](https://doc.rust-lang.org/std/convert/index.html) module.
The traits are the following:

- `From` and `Into` covered in [`from_into`](from_into.rs)
- `TryFrom` and `TryInto` covered in [`try_from_into`](try_from_into.rs)
- `AsRef` and `AsMut` covered in [`as_ref_mut`](as_ref_mut.rs)

Furthermore, the `std::str` module offers a trait called [`FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html) which helps with converting strings into target types via the `parse` method on strings. If properly implemented for a given type `Person`,
then `let p: Person = "Mark,20".parse().unwrap()` should both compile and run without panicking.

These should be the main ways ***within the standard library*** to convert data into your desired types.

- solutions: [rustlings-solutions-5/conversions at main · gaveen/rustlings-solutions-5](https://github.com/gaveen/rustlings-solutions-5/tree/main/conversions)

## Further information

These are not directly covered in the book, but the standard library has a great documentation for it.

- [conversions](https://doc.rust-lang.org/std/convert/index.html)
- [`FromStr` trait](https://doc.rust-lang.org/std/str/trait.FromStr.html)

## Rustlings

### as_ref_mut

> `AsRef` and `AsMut` allow for cheap reference-to-reference conversions.

Read more about them at [AsRef in std::convert - Rust](https://doc.rust-lang.org/std/convert/trait.AsRef.html)
and [AsMut in std::convert - Rust](https://doc.rust-lang.org/std/convert/trait.AsMut.html), respectively.

~~~admonish note title="as_ref_mut" collapsible=true
```rust,editable
{{#include as_ref_mut.rs}}
```
~~~

~~~admonish tip title="Hint" collapsible=true
Add AsRef<str> as a trait bound to the functions.
~~~

> compare to [traits4](https://kuanhsiaokuo.github.io/the-rustlings-collection/traits/traits.html)

~~~admonish success title="solution1: generic trait bounds" collapsible=true
```rust,editable
// Obtain the number of bytes (not characters) in the given argument
// Add the AsRef trait appropriately as a trait bound
fn byte_counter<T: AsRef<str>>(arg: T) -> usize {
    arg.as_ref().as_bytes().len()
}

// Obtain the number of characters (not bytes) in the given argument
// Add the AsRef trait appropriately as a trait bound
fn char_counter<T: AsRef<str>>(arg: T) -> usize {
    arg.as_ref().chars().count()
}

// Squares a number using AsMut. Add the trait bound as is appropriate and
// implement the function body.
fn num_sq<T: AsMut<u32>>(arg: &mut T) {
    *arg.as_mut() *= *arg.as_mut()
}
```
~~~

~~~admonish success title="solution2: impl trait bounds" collapsible=true
```rust,editable
// Obtain the number of bytes (not characters) in the given argument
// Add the AsRef trait appropriately as a trait bound
fn byte_counter(arg: impl AsRef<str>) -> usize {
    arg.as_ref().as_bytes().len()
}

// Obtain the number of characters (not bytes) in the given argument
// Add the AsRef trait appropriately as a trait bound
fn char_counterd(arg: impl AsRef<str>) -> usize {
    arg.as_ref().chars().count()
}

// Squares a number using AsMut. Add the trait bound as is appropriate and
// implement the function body.
fn num_sq(arg: &mut impl AsRef<str>) {
    *arg.as_mut() *= *arg.as_mut()
}
```
~~~

~~~admonish success title="solution3: where clause" collapsible=true
```rust,editable
// Obtain the number of bytes (not characters) in the given argument
// Add the AsRef trait appropriately as a trait bound
fn byte_counter<T>(arg: T) -> usize
where T: AsRef<str>
{
    arg.as_ref().as_bytes().len()
}
```
~~~

### from_into

~~~admonish note title="from_into" collapsible=true
```rust,editable
{{#include from_into.rs}}
```
~~~

### from_str

~~~admonish note title="from_str" collapsible=true
```rust,editable
{{#include from_str.rs}}
```
~~~

### try_from_into

~~~admonish note title="try_from_into" collapsible=true
```rust,editable
{{#include try_from_into.rs}}
```
~~~

### using_as

~~~admonish note title="using_as" collapsible=true
```rust,editable
{{#include using_as.rs}}
```
~~~

