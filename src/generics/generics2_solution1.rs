// This powerful wrapper provides the ability to store a positive integer value.
// Rewrite it using generics so that it supports wrapping ANY type.

struct Wrapper { // fix here
    value: T,
}

impl<T> Wrapper<T> {
    pub fn new(value: T) -> Self {
        Wrapper { value }
    }
}

fn main() {
    assert_eq!(Wrapper::new(42).value, 42);
    assert_eq!(Wrapper::new("Foo").value, "Foo");
}