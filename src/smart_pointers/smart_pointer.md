# Smart Pointers

<!--ts-->

* [Smart Pointers](#smart-pointers)
    * [Further Information](#further-information)
    * [Rustlings](#rustlings)
        * [Arc](#arc)
        * [Box](#box)
        * [Cow: Clone-On-Write](#cow-clone-on-write)
        * [Rc](#rc)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Wed Mar  1 12:12:53 UTC 2023 -->

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

~~~admonish success title="solution: Box::new()" collapsible=true
```rust, editable
#[derive(PartialEq, Debug)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

pub fn create_empty_list() -> List {
    List::Nil
}

pub fn create_non_empty_list() -> List {
    List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))))
}
```
~~~

### Cow: Clone-On-Write

> This exercise explores the Cow, or Clone-On-Write type.

- Cow is a clone-on-write smart pointer.
- It can enclose and provide immutable access to borrowed data, and clone the data lazily when mutation or ownership is required.
- The type is designed to work with general borrowed data via the Borrow trait.

> This exercise is meant to show you what to expect when passing data to Cow.

~~~admonish note title="cow1" collapsible=true
```rust,editable
{{#include cow1.rs}}
```
~~~

~~~admonish tip title="Hint" collapsible=true
Since the vector is already owned, the `Cow` type doesn't need to clone it.

Checkout [Cow in std::borrow - Rust](https://doc.rust-lang.org/std/borrow/enum.Cow.html) for documentation
on the `Cow` type.
~~~

~~~admonish success title="solution: Cow::Borrowed(_)/Cow::Owned(_)" collapsible=true
```rust, editable
// cow1.rs

// This exercise explores the Cow, or Clone-On-Write type.
// Cow is a clone-on-write smart pointer.
// It can enclose and provide immutable access to borrowed data, and clone the data lazily when mutation or ownership is required.
// The type is designed to work with general borrowed data via the Borrow trait.

use std::borrow::Cow;

fn abs_all<'a, 'b>(input: &'a mut Cow<'b, [i32]>) -> &'a mut Cow<'b, [i32]> {
    for i in 0..input.len() {
        let v = input[i];
        if v < 0 {
            // Clones into a vector if not already owned.
            input.to_mut()[i] = -v;
        }
    }
    input
}

fn main() {
    // No clone occurs because `input` doesn't need to be mutated.
    let slice = [0, 1, 2];
    let mut input = Cow::from(&slice[..]);
    match abs_all(&mut input) {
        Cow::Borrowed(_) => println!("I borrowed the slice!"),
        _ => panic!("expected borrowed value"),
    }

    // Clone occurs because `input` needs to be mutated.
    let slice = [-1, 0, 1];
    let mut input = Cow::from(&slice[..]);
    match abs_all(&mut input) {
        Cow::Owned(_) => println!("I modified the slice and now own it!"),
        _ => panic!("expected owned value"),
    }

    // No clone occurs because `input` is already owned.
    let slice = vec![-1, 0, 1];
    let mut input = Cow::from(slice);
    match abs_all(&mut input) {
        Cow::Owned(_) => println!("I own this slice!"),
        _ => panic!("expected borrowed value"),
    }
}
```
~~~

### Rc

> In this exercise, we want to express the concept of multiple owners via the Rc<T> type.

- This is a model of our solar system - there is a Sun type and multiple Planets.
- The Planets take ownership of the sun, indicating that they revolve around the sun.

> Make this code compile by using the proper Rc primitives to express that the sun has multiple owners.

~~~admonish note title="rc1" collapsible=true
```rust,editable
{{#include rc1.rs}}
```
~~~

~~~admonish tip title="Hint" collapsible=true
This is a straightforward exercise to use the Rc<T> type:
- Each Planet has ownership of the Sun, and uses Rc::clone() to increment the reference count of the Sun.
- After using drop() to move the Planets out of scope individually, the reference count goes down.
- In the end the sun only has one reference again, to itself. 

> See more at:
[Rc<T>(Reference Counting): single-threaded scenarios, immutable references of multiple owners, the Reference Counted Smart Pointer - The Rust Programming Language](https://kuanhsiaokuo.github.io/the-rust-programming-book-khk/ch15-04-rc.html)

* Unfortunately Pluto is no longer considered a planet :(
~~~

~~~admonish success title="solution: Arc::new()" collapsible=true
```rust,editable
use std::rc::Rc;

#[derive(Debug)]
struct Sun {}

#[derive(Debug)]
enum Planet {
    Mercury(Rc<Sun>),
    Venus(Rc<Sun>),
    Earth(Rc<Sun>),
    Mars(Rc<Sun>),
    Jupiter(Rc<Sun>),
    Saturn(Rc<Sun>),
    Uranus(Rc<Sun>),
    Neptune(Rc<Sun>),
}

impl Planet {
    fn details(&self) {
        println!("Hi from {:?}!", self)
    }
}

fn main() {
    let sun = Rc::new(Sun {});
    println!("reference count = {}", Rc::strong_count(&sun)); // 1 reference

    let mercury = Planet::Mercury(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 2 references
    mercury.details();

    let venus = Planet::Venus(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 3 references
    venus.details();

    let earth = Planet::Earth(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 4 references
    earth.details();

    let mars = Planet::Mars(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 5 references
    mars.details();

    let jupiter = Planet::Jupiter(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 6 references
    jupiter.details();

    let saturn = Planet::Saturn(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 7 references
    saturn.details();

    let uranus = Planet::Uranus(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 8 references
    uranus.details();

    let neptune = Planet::Neptune(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 9 references
    neptune.details();

    assert_eq!(Rc::strong_count(&sun), 9);

    drop(neptune);
    println!("reference count = {}", Rc::strong_count(&sun)); // 8 references

    drop(uranus);
    println!("reference count = {}", Rc::strong_count(&sun)); // 7 references

    drop(saturn);
    println!("reference count = {}", Rc::strong_count(&sun)); // 6 references

    drop(jupiter);
    println!("reference count = {}", Rc::strong_count(&sun)); // 5 references

    drop(mars);
    println!("reference count = {}", Rc::strong_count(&sun)); // 4 references

    drop(earth);
    println!("reference count = {}", Rc::strong_count(&sun)); // 3 references

    drop(venus);
    println!("reference count = {}", Rc::strong_count(&sun)); // 2 references

    drop(mercury);
    println!("reference count = {}", Rc::strong_count(&sun)); // 1 reference

    assert_eq!(Rc::strong_count(&sun), 1);
}
```
~~~

### Memory Leak

~~~admonish note title="memory_leak1" collapsible=true
```rust,editable
{{#include memory_leak1.rs}}
```
~~~
