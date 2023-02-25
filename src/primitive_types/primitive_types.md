# Primitive Types

Rust has a couple of basic types that are directly implemented into the
compiler. In this section, we'll go through the most important ones.

## Further information

- [Data Types](https://doc.rust-lang.org/stable/book/ch03-02-data-types.html)
- [The Slice Type](https://doc.rust-lang.org/stable/book/ch04-03-slices.html)

## Rustlings

~~~admonish note title="primitive_types1: Fill in the rest of the line that has code missing!" collapsible=true
```rust,editable
{{#include primitive_types1.rs}}
```
~~~

---

~~~admonish note title="primitive_types2: Fill in the rest of the line that has code missing!" collapsible=true
```rust,editable
{{#include primitive_types2.rs}}
```
~~~

~~~admonish tip title="Hint: single or double quotes, they are different" collapsible=true
'C' is str, but "C" is &str
~~~

---

~~~admonish note title="primitive_types3: Create an array with at least 100 elements in it where the ??? is." collapsible=true
```rust,editable
{{#include primitive_types3.rs}}
```
~~~

~~~admonish tip title="Hint" collapsible=true
There's a shorthand to initialize Arrays with a certain size that does not
require you to type in 100 items (but you certainly can if you want!).
For example, you can do:
let array = ["Are we there yet?"; 10];

Bonus: what are some other things you could have that would return true
for `a.len() >= 100`?

~~~

~~~admonish success title="solution" collapsible=true
```rust,editable
{{#include primitive_types3_solution.rs}}
```
~~~

---

~~~admonish note title="primitive_types4: Get a slice out of Array a " collapsible=true
```rust,editable
{{#include primitive_types4.rs}}
```
~~~

---

~~~admonish note title="primitive_types5: Destructure the *cat* tuple" collapsible=true
```rust,editable
{{#include primitive_types5.rs}}
```
~~~

---

~~~admonish note title="primitive_types6: Use a tuple index to access the second element of *numbers*." collapsible=true
```rust,editable
{{#include primitive_types6.rs}}
```
~~~
