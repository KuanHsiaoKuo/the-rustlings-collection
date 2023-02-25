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

- [Traits](https://kuanhsiaokuo.github.io/the-rust-programming-book-khk/ch10-02-traits.html#specifying-multiple-trait-bounds-with-the--syntax)
- [rustlings-solutions-5/traits at main · gaveen/rustlings-solutions-5](https://github.com/gaveen/rustlings-solutions-5/tree/main/traits)

## Rustlings

~~~admonish note title="traits1: Time to implement some traits!" collapsible=true
```rust,editable
{{#include traits1.rs}}
```
~~~

~~~admonish tip title="Hint" collapsible=true
A discussion about Traits in Rust can be found at:
[🪐Traits: Defining Shared Behavior - The Rust Programming Language](https://kuanhsiaokuo.github.io/the-rust-programming-book-khk/ch10-02-traits.html)
~~~

~~~admonish bug title="solution: String" collapsible=true
```rust,editable
impl AppendBar for String {
    //Add your code here
    fn append_bar(self) -> Self {
        format!("{}Bar", self)
        // or
        // format!("{self}Bar")
        // or
        // self + "Bar"
    }
}
```
~~~

---

~~~admonish note title="traits2: implement the trait for a vector of strings." collapsible=true
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

~~~admonish bug title="solution: Vec<String> " collapsible=true
```rust, editable
// Add your code here
impl AppendBar for Vec<String> {
    fn append_bar(mut self) -> Self { // Borrow self as `mut`
        self.push("Bar".to_string());
        self
    }
}
```
~~~

---

~~~admonish note title="traits3: implement the Licensed trait for both structures and have them return the same information without writing the same function twice." collapsible=true
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

~~~admonish bug title="solution: Default Trait Method" collapsible=true
```rust, editable
pub trait Licensed {
    fn licensing_info(&self) -> String {
        String::from("Some information")
    }
}
```
~~~

---

~~~admonish note title="traits4: trait bounds" collapsible=true
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

~~~admonish bug title="solution: impl" collapsible=true
```rust, editable
fn compare_license_types(software: impl Licensed, software_two: impl Licensed) -> bool {
    software.licensing_info() == software_two.licensing_info()
}
```
~~~

---

~~~admonish note title="traits5: impl multiple traits" collapsible=true
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

~~~admonish bug title="solution: impl or where" collapsible=true
```rust,editable
fn some_func<T>(item: T) -> bool
    where T: SomeTrait + OtherTrait
{
    item.some_function() && item.other_function()
}

fn some_func(item: impl SomeTrait + OtherTrait) -> bool {
    item.some_function() && item.other_function()
}

```
~~~
