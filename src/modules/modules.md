# Modules

In this section we'll give you an introduction to Rust's module system.

## Further information

- [The Module System](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)

## Rustlings

~~~admonish note title="modules1: pub" collapsible=true
```rust,editable
{{#include modules1.rs}}
```
~~~

~~~admonish tip title="Hint" collapsible=true
> add `pub`
Everything is private in Rust by default-- but there's a keyword we can use
to make something public! The compiler error should point to the thing that
needs to be public.
~~~

~~~admonish note title="modules2: use self::xxx as bbb" collapsible=true
```rust,editable
{{#include modules2.rs}}
```
~~~

~~~admonish tip title="Hint" collapsible=true
The delicious_snacks module is trying to present an external interface that is
different than its internal structure (the `fruits` and `veggies` modules and
associated constants). Complete the `use` statements to fit the uses in main and
find the one keyword missing for both constants.

```rust
    pub use self::fruits::PEAR as fruit;
    pub use self::veggies::CUCUMBER as veggie;
```

~~~

~~~admonish note title="modules3" collapsible=true
```rust,editable
{{#include modules3.rs}}
```
~~~

~~~admonish tip title="Hint" collapsible=true
```rust
use std::time::{SystemTime, UNIX_EPOCH};
```
UNIX_EPOCH and SystemTime are declared in the std::time module. Add a use statement
for these two to bring them into scope. You can use nested paths or the glob
operator to bring these two in using only one line.
~~~
