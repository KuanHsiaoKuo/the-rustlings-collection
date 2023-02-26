# Vectors

Vectors are one of the most-used Rust data structures. In other programming
languages, they'd simply be called Arrays, but since Rust operates on a
bit of a lower level, an array in Rust is stored on the stack (meaning it
can't grow or shrink, and the size needs to be known at compile time),
and a Vector is stored in the heap (where these restrictions do not apply).

Vectors are a bit of a later chapter in the book, but we think that they're
useful enough to talk about them a bit earlier. We shall be talking about
the other useful data structure, hash maps, later.

## Further information

- [Vectors: Storing Lists of The Same Type of Values - The Rust Programming Language](https://kuanhsiaokuo.github.io/the-rust-programming-book-khk/ch08-01-vectors.html)
- [`iter_mut`](https://doc.rust-lang.org/std/primitive.slice.html#method.iter_mut)
- [`map`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map)
- [rustlings-solutions-5/vecs at main · gaveen/rustlings-solutions-5](https://github.com/gaveen/rustlings-solutions-5/tree/main/vecs)

## Rustlings

~~~admonish note title="vecs1: [i32; 4] or Vec<i32>" collapsible=true
```rust,editable
{{#include vecs1.rs}}
```
~~~

~~~admonish tip title="Hint" collapsible=true
In Rust, there are two ways to define a Vector.
1. One way is to use the `Vec::new()` function to create a new vector
  and fill it with the `push()` method.
2. The second way, which is simpler is to use the `vec![]` macro and
  define your elements inside the square brackets.
Check this chapter: [Vectors: Storing Lists of The Same Type of Values - The Rust Programming Language](https://kuanhsiaokuo.github.io/the-rust-programming-book-khk/ch08-01-vectors.html)
of the Rust book to learn more.
~~~

~~~admonish success title="solution1" collapsible=true
```rust, editable
let v = vec![10, 20, 30, 40];
```
~~~

---

~~~admonish note title="vecs2" collapsible=true
```rust,editable
{{#include vecs2.rs}}
```
~~~

~~~admonish tip title="Hint" collapsible=true
Hint 1: `i` is each element from the Vec as they are being iterated. Can you try
multiplying this?

Hint 2: For the first function, there's a way to directly access the numbers stored
in the Vec, using the * dereference operator. You can both access and write to the
number that way.

After you've completed both functions, decide for yourself which approach you like
better. What do you think is the more commonly used pattern under Rust developers?
~~~

~~~admonish success title="solution1" collapsible=true
```rust, editable
fn vec_loop(mut v: Vec<i32>) -> Vec<i32> {
    for i in v.iter_mut() {
      *i *= 2;
    }

    // At this point, `v` should be equal to [4, 8, 12, 16, 20].
    v
}

fn vec_map(v: &Vec<i32>) -> Vec<i32> {
    v.iter().map(|num| {
        num*2
    }).collect()
}
```
~~~
