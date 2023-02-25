# Generics

Generics is the topic of generalizing types and functionalities to broader cases.
This is extremely useful for reducing code duplication in many ways, but can call for rather involving syntax.
Namely, being generic requires taking great care to specify over which types a generic type is actually considered valid.
The simplest and most common use of generics is for type parameters.

## Further information

- [Generic Data Types](https://doc.rust-lang.org/stable/book/ch10-01-syntax.html)
- [Bounds](https://doc.rust-lang.org/rust-by-example/generics/bounds.html)

## Rustlings

~~~admonish failure title="generics1" collapsible=true
```rust,editable
{{#include generics1.rs}}
```
~~~

> [rustlings-solutions-5/generics at main · gaveen/rustlings-solutions-5](https://github.com/gaveen/rustlings-solutions-5/tree/main/generics)

~~~admonish tip title="Hint" collapsible=true
Vectors in Rust make use of generics to create dynamically sized arrays of any type.
You need to tell the compiler what type we are pushing onto this vector.
~~~

~~~admonish success title="solution1: &str" collapsible=true
```rust,editable
{{#include generics1_solution1.rs}}
```
~~~

~~~admonish bug title="solution2: String" collapsible=true
```rust,editable
{{#include generics1_solution2.rs}}
```
~~~

---

~~~admonish failure title="generics2" collapsible=true
```rust,editable
{{#include generics2.rs}}
```
~~~

~~~admonish tip title="Hint" collapsible=true
Currently we are wrapping only values of type 'u32'.
Maybe we could update the explicit references to this data type somehow?

If you are still stuck, please read:
> [✨Generic Data Types - The Rust Programming Language](https://kuanhsiaokuo.github.io/the-rust-programming-book-khk/ch10-01-syntax.html#in-method-definitions)
~~~

~~~admonish bug title="solution1: convert to use generics" collapsible=true
```rust,editable
{{#include generics2_solution1.rs}}
```
~~~