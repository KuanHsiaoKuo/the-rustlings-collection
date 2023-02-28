# Strings: `&str` for string slice and `String` for owned string

Rust has two string types, a string slice (`&str`) and an owned string (`String`).
We're not going to dictate when you should use which one, but we'll show you how
to identify and create them, as well as use them.

## Further information

- [Strings: Storing UTF-8 Encoded Text - The Rust Programming Language](https://kuanhsiaokuo.github.io/the-rust-programming-book-khk/ch08-02-strings.html)
- [rustlings-solutions-5/strings at main · gaveen/rustlings-solutions-5](https://github.com/gaveen/rustlings-solutions-5/tree/main/strings)

## Rustlings

~~~admonish note title="strings1: String -> to_string()" collapsible=true
```rust,editable
{{#include strings1.rs}}
```
~~~

~~~admonish tip title="Hint" collapsible=true
The `current_favorite_color` function is currently returning a string slice with the `'static`
lifetime. 

We know this because the data of the string lives in our code itself -- it doesn't
come from a file or user input or another program -- so it will live as long as our program
lives. 

But it is still a `string slice`. There's one way to create a `String` by converting a
string slice covered in the Strings chapter of the book, and another way that uses the `From`
trait.

~~~

~~~admonish success title="solution1: to_string" collapsible=true
```rust,editable
fn current_favorite_color() -> String {
    "blue".to_string()
}
```
~~~

~~~admonish success title="solution2: From trait" collapsible=true
```rust,editable
fn current_favorite_color() -> String {
    String::from("blue")
}
```
~~~

---

~~~admonish note title="strings2: &str -> as_str()" collapsible=true
```rust,editable
{{#include strings2.rs}}
```
~~~

~~~admonish tip title="Hint" collapsible=true
Yes, it would be really easy to fix this by just changing the value bound to `word` to be a
string slice instead of a `String`, wouldn't it?? There is a way to add one character to line
9, though, that will coerce the `String` into a string slice.
~~~

~~~admonish success title="solution: as_str()" collapsible=true
```rust,editable
is_a_color_word(word.as_str())
```
~~~

~~~admonish note title="strings3" collapsible=true
```rust,editable
{{#include strings3.rs}}
```
~~~

~~~admonish tip title="Hint" collapsible=true
There's tons of useful standard library functions for strings. Let's try and use some of
them: [String in std::string - Rust](https://doc.rust-lang.org/std/string/struct.String.html#method.trim)!

For the compose_me method: You can either use the `format!` macro, or convert the string
slice into an owned string, which you can then freely extend.
~~~

~~~admonish success title="solution" collapsible=true
```rust, editable
fn trim_me(input: &str) -> String {
    // Remove whitespace from the beginning and the end of a string!
    input.trim().to_string()
}

fn compose_me(input: &str) -> String {
    // Add  to the string! There's multiple ways to do this!
    input.to_owned() + " world!"
}

fn replace_me(input: &str) -> String {
    // Replace "cars" in the string with "balloons"!
    input.replace("cars", "balloons")
}
```
~~~

~~~admonish note title="strings4" collapsible=true
```rust,editable
{{#include strings4.rs}}
```
~~~

~~~admonish tip title="No Hint" collapsible=true
~~~

~~~admonish success title="solution: distinguish *String* and *&str*" collapsible=true
```rust, editable
fn main() {
    string_slice("blue");
    string("red".to_string());
    string(String::from("hi"));
    string("rust is fun!".to_owned());
    string("nice weather".into());
    string(format!("Interpolation {}", "Station"));
    string_slice(&String::from("abc")[0..1]);
    string_slice("  hello there ".trim());
    string("Happy Monday!".to_string().replace("Mon", "Tues"));
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
```
~~~
