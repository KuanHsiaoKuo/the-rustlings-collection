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
<!-- Added by: runner, at: Sat Feb 25 10:36:28 UTC 2023 -->

<!--te-->
In Rust, smart pointers are variables that contain an address in memory and reference some other data, but they also have additional metadata and capabilities.
Smart pointers in Rust often own the data they point to, while references only borrow data.

## Further Information

- [Smart Pointers](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html)
- [Using Box to Point to Data on the Heap](https://doc.rust-lang.org/book/ch15-01-box.html)
- [Rc\<T\>, the Reference Counted Smart Pointer](https://doc.rust-lang.org/book/ch15-04-rc.html)
- [Shared-State Concurrency](https://doc.rust-lang.org/book/ch16-03-shared-state.html)
- [Cow Documentation](https://doc.rust-lang.org/std/borrow/enum.Cow.html)

## Rustlings

### Arc

~~~admonish note title="arc1" collapsible=true
```rust,editable
{{#include arc1.rs}}
```
~~~

### Box

~~~admonish note title="box1" collapsible=true
```rust,editable
{{#include box1.rs}}
```
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