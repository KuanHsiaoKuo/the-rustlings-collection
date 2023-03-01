# Smart Pointers

<!--ts-->

* [Smart Pointers](#smart-pointers)
    * [Further Information](#further-information)
    * [Rustlings](#rustlings)
        * [Arc](#arc)
        * [Box](#box)
        * [Cow](#cow)
        * [Rc](#rc)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Wed Mar  1 11:51:26 UTC 2023 -->

<!--te-->
In Rust, smart pointers are variables that contain an address in memory and reference some other data, but they also have additional metadata and capabilities.
Smart pointers in Rust often own the data they point to, while references only borrow data.

## Further Information

- [✨Smart Pointers: Heap、Deref、Drop、Rc、RefCell、Reference Cycle - The Rust Programming Language](https://kuanhsiaokuo.github.io/the-rust-programming-book-khk/ch15-00-smart-pointers.html)
- [Heap: Using Box<T> to Point to Data on the Heap - The Rust Programming Language](https://kuanhsiaokuo.github.io/the-rust-programming-book-khk/ch15-01-box.html)
- [Rc<T>(Reference Counting): single-threaded scenarios, immutable references of multiple owners, the Reference Counted Smart Pointer - The Rust Programming Language](https://kuanhsiaokuo.github.io/the-rust-programming-book-khk/ch15-04-rc.html)
- [Shared-State Concurrency - The Rust Programming Language](https://kuanhsiaokuo.github.io/the-rust-programming-book-khk/ch16-03-shared-state.html)
- [Cow Documentation](https://doc.rust-lang.org/std/borrow/enum.Cow.html)

> [rustlings-solutions-5/standard_library_types at main · gaveen/rustlings-solutions-5](https://github.com/gaveen/rustlings-solutions-5/tree/main/standard_library_types)

## Rustlings

### Arc

1. In this exercise, we are given a Vec of u32 called "numbers" with values ranging
   from 0 to 99 -- [ 0, 1, 2, ..., 98, 99 ]
2. We would like to use this set of numbers within 8 different threads simultaneously.
3. Each thread is going to get the sum of every eighth value, with an offset.

- The first thread (offset 0), will sum 0, 8, 16, ...
- The second thread (offset 1), will sum 1, 9, 17, ...
- The third thread (offset 2), will sum 2, 10, 18, ...
- ...
- The eighth thread (offset 7), will sum 7, 15, 23, ...

> Because we are using threads, our values need to be `thread-safe`. Therefore,
> we are using Arc. We need to make a change in each of the two TODOs.

1. Make this code compile by filling in a value for `shared_numbers` where the
   first TODO comment is
2. and create an initial binding for `child_numbers` where the second TODO comment is.

> Try not to create any copies of the `numbers` Vec!

~~~admonish bug title="arc1: Using Arc to keep *thread-safe*" collapsible=true
```rust,editable
{{#include arc1.rs}}
```
~~~

~~~admonish tip title="Hint" collapsible=true
1. Make `shared_numbers` be an `Arc` from the numbers vector. 
2. Then, in order
to avoid creating a copy of `numbers`, you'll need to create `child_numbers`
inside the loop but still in the main thread.
3. `child_numbers` should be a clone of the Arc of the numbers instead of a
thread-local copy of the numbers.

> This is a simple exercise if you understand the underlying concepts, but if this
is too much of a struggle, consider reading through all of Concurrency Chapter in the book:
[Fearless Concurrency - The Rust Programming Language](https://kuanhsiaokuo.github.io/the-rust-programming-book-khk/ch16-00-concurrency.html)
~~~

~~~admonish success title="solution: Arc::new()" collapsible=true
```rust,editable
#![forbid(unused_imports)] // Do not change this, (or the next) line.
use std::sync::Arc;
use std::thread;

fn main() {
    let numbers: Vec<_> = (0..100u32).collect();
    let shared_numbers = Arc::new(numbers); // filling in a value for `shared_numbers`
    let mut joinhandles = Vec::new();

    for offset in 0..8 {
        let child_numbers = shared_numbers.clone(); // create an initial binding for `child_numbers`
        joinhandles.push(thread::spawn(move || {
            let sum: u32 = child_numbers.iter().filter(|n| *n % 8 == offset).sum();
            println!("Sum of offset {} is {}", offset, sum);
        }));
    }
    for handle in joinhandles.into_iter() {
        handle.join().unwrap();
    }
}
```
~~~

### Box

> At compile time, Rust needs to know `how much space a type takes up`. This becomes problematic
> for `recursive types`, where a value can have as part of itself another value of the same type.

To get around the issue, we can use a `Box` - a smart pointer used to store data on the heap,
which also allows us to wrap a recursive type.

The recursive type we're implementing in this exercise is the `cons list` - a data structure
frequently found in functional programming languages. Each item in a cons list contains two
elements: the value of the current item and the next item. The last item is a value called `Nil`.

- Step 1: use a `Box` in the enum definition to make the code compile
- Step 2: create both empty and non-empty cons lists by replacing `todo!()`

~~~admonish note title="box1" collapsible=true
```rust,editable
{{#include box1.rs}}
```
~~~

~~~admonish tip title="Hint" collapsible=true
### Step 1
The compiler's message should help: since we cannot store the value of the actual type
when working with recursive types, we need to store a reference (pointer) to its value.
We should, therefore, place our `List` inside a `Box`. 

> More details in the book here: [Heap: Using Box<T> to Point to Data on the Heap - The Rust Programming Language](https://kuanhsiaokuo.github.io/the-rust-programming-book-khk/ch15-01-box.html#enabling-recursive-types-with-boxes)

### Step 2
Creating an empty list should be fairly straightforward (hint: peek at the assertions).
For a non-empty list keep in mind that we want to use our Cons "list builder".
Although the current list is one of integers (i32), feel free to change the definition
and try other types!
~~~

### Cow

~~~admonish note title="cow1" collapsible=true
```rust,editable
{{#include cow1.rs}}
```
~~~

### Rc

~~~admonish note title="rc1" collapsible=true
```rust,editable
{{#include rc1.rs}}
```
~~~