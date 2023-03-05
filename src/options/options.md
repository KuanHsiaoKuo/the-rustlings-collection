# Options

Type `Option` represents an optional value: every Option is either Some and contains a value, or None, and does not.

> Option types are very common in Rust code, as they have a number of uses:

- Initial values
- Return values for functions that are not defined over their entire input range (partial functions)
- Return value for otherwise reporting simple errors, where None is returned on error
- Optional struct fields
- Struct fields that can be loaned or "taken"
- Optional function arguments
- Nullable pointers
- Swapping things out of difficult situations

## Further Information

- [✨Generic Data Types->Option Enum Format - The Rust Programming Language](https://kuanhsiaokuo.github.io/the-rust-programming-book-khk/ch10-01-syntax.html#in-enum-definitions)
- [Option Module Documentation](https://doc.rust-lang.org/std/option/)
- [Option Enum Documentation](https://doc.rust-lang.org/std/option/enum.Option.html)
- [if let - The Rust Programming Language](https://kuanhsiaokuo.github.io/the-rust-programming-book-khk/rust_by_example_src/flow_control/if_let.html)
- [while let - The Rust Programming Language](https://kuanhsiaokuo.github.io/the-rust-programming-book-khk/rust_by_example_src/flow_control/while_let.html)

> [rustlings-solutions-5/options at main · gaveen/rustlings-solutions-5](https://github.com/gaveen/rustlings-solutions-5/tree/main/options)

## Rustlings

~~~admonish failure title="options1" collapsible=true
```rust,editable
{{#include options1.rs}}
```
~~~

~~~admonish tip title="Hint" collapsible=true
Options can have a Some value, with an inner value, or a None value, without an inner value.

There's multiple ways to get at the inner value, you can use `unwrap`, or `pattern match`. 

> Unwrapping is the easiest, but how do you do it safely so that it doesn't panic in your face later?
~~~

~~~admonish success title="solution1: pattern match " collapsible=true
```rust, editable
fn maybe_icecream(time_of_day: u16) -> Option<u16> {
    // We use the 24-hour system here, so 10PM is a value of 22
    // The Option output should gracefully handle cases where time_of_day > 24.
    match time_of_day {
        0..=21 => Some(5),
        22..=24 => Some(0),
        _ => None,
    }
    // Exclusive range (e.g., 0..22) pattern use here is experimental
    // on rustc 1.62.1
}
```

```rust, editable
assert_eq!(icecreams.unwrap_or(0), 5); // Use unwrapped Some or 0 
```

~~~

---

~~~admonish failure title="options2" collapsible=true
```rust,editable
{{#include options2.rs}}
```
~~~

~~~admonish tip title="Hint" collapsible=true
check out:

- [if let - The Rust Programming Language](https://kuanhsiaokuo.github.io/the-rust-programming-book-khk/rust_by_example_src/flow_control/if_let.html)
- [while let - The Rust Programming Language](https://kuanhsiaokuo.github.io/the-rust-programming-book-khk/rust_by_example_src/flow_control/while_let.html)

Remember that Options can be stacked in if let and while let.
For example: Some(Some(variable)) = variable2
Also see Option::flatten
~~~

~~~admonish success title="solution: if let & while let" collapsible=true
```rust, editable
        if let Some(word) = optional_target {
            assert_eq!(word, target);
        }
```
```rust, editable
        while let Some(integer) = optional_integers.pop().flatten() {
            assert_eq!(integer, range);
            range -= 1;
        }
```
~~~

---

~~~admonish failure title="options3: ref -> value partially moved here" collapsible=true
```rust,editable
{{#include options3.rs}}
```
~~~

~~~admonish tip title="Hint" collapsible=true
The compiler says a `partial move` happened in the `match`
statement.

How can this be avoided? The compiler shows the correction needed.
After making the correction as suggested by the compiler, do
read: [ref - Rust](https://doc.rust-lang.org/std/keyword.ref.html)
~~~

~~~admonish success title="solution: ref " collapsible=true
```rust, editable
Some(ref p)
```
~~~
