struct Wrapper {
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