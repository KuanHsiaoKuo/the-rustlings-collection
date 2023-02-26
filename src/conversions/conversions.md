# Type conversions

<!--ts-->
* [Type conversions](#type-conversions)
   * [Further information](#further-information)
   * [Rustlings](#rustlings)
      * [as_ref_mut](#as_ref_mut)
      * [from_into](#from_into)
      * [from_str](#from_str)
      * [try_from_into](#try_from_into)
      * [using_as](#using_as)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Sun Feb 26 04:15:34 UTC 2023 -->

<!--te-->
Rust offers a multitude of ways to convert a value of a given type into another type.

The simplest form of type conversion is a type cast expression. It is denoted with the binary operator `as`. For instance, `println!("{}", 1 + 1.0);` would not compile, since `1` is an integer while `1.0` is a float. However, `println!("{}", 1 as f32 + 1.0)` should compile. The
exercise [`using_as`](using_as.rs) tries to cover this.

Rust also offers traits that facilitate type conversions upon implementation. These traits can be found under the [`convert`](https://doc.rust-lang.org/std/convert/index.html) module.
The traits are the following:

- `From` and `Into` covered in [`from_into`](from_into.rs)
- `TryFrom` and `TryInto` covered in [`try_from_into`](try_from_into.rs)
- `AsRef` and `AsMut` covered in [`as_ref_mut`](as_ref_mut.rs)

Furthermore, the `std::str` module offers a trait called [`FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html) which helps with converting strings into target types via the `parse` method on strings. If properly implemented for a given type `Person`,
then `let p: Person = "Mark,20".parse().unwrap()` should both compile and run without panicking.

These should be the main ways ***within the standard library*** to convert data into your desired types.

- solutions: [rustlings-solutions-5/conversions at main · gaveen/rustlings-solutions-5](https://github.com/gaveen/rustlings-solutions-5/tree/main/conversions)

## Further information

These are not directly covered in the book, but the standard library has a great documentation for it.

- [conversions](https://doc.rust-lang.org/std/convert/index.html)
- [`FromStr` trait](https://doc.rust-lang.org/std/str/trait.FromStr.html)

## Rustlings

### as_ref_mut

> `AsRef` and `AsMut` allow for cheap reference-to-reference conversions.

Read more about them at [AsRef in std::convert - Rust](https://doc.rust-lang.org/std/convert/trait.AsRef.html)
and [AsMut in std::convert - Rust](https://doc.rust-lang.org/std/convert/trait.AsMut.html), respectively.

~~~admonish note title="as_ref_mut: add appropriate trait bound" collapsible=true
```rust,editable
{{#include as_ref_mut.rs}}
```
~~~

~~~admonish tip title="Hint" collapsible=true
Add AsRef<str> as a trait bound to the functions.
~~~

> compare to [traits4](https://kuanhsiaokuo.github.io/the-rustlings-collection/traits/traits.html)

~~~admonish success title="solution1: generic trait bounds" collapsible=true
```rust,editable
// Obtain the number of bytes (not characters) in the given argument
// Add the AsRef trait appropriately as a trait bound
fn byte_counter<T: AsRef<str>>(arg: T) -> usize {
    arg.as_ref().as_bytes().len()
}

// Obtain the number of characters (not bytes) in the given argument
// Add the AsRef trait appropriately as a trait bound
fn char_counter<T: AsRef<str>>(arg: T) -> usize {
    arg.as_ref().chars().count()
}

// Squares a number using AsMut. Add the trait bound as is appropriate and
// implement the function body.
fn num_sq<T: AsMut<u32>>(arg: &mut T) {
    *arg.as_mut() *= *arg.as_mut()
}
```
~~~

~~~admonish success title="solution2: impl trait bounds" collapsible=true
```rust,editable
// Obtain the number of bytes (not characters) in the given argument
// Add the AsRef trait appropriately as a trait bound
fn byte_counter(arg: impl AsRef<str>) -> usize {
    arg.as_ref().as_bytes().len()
}

// Obtain the number of characters (not bytes) in the given argument
// Add the AsRef trait appropriately as a trait bound
fn char_counterd(arg: impl AsRef<str>) -> usize {
    arg.as_ref().chars().count()
}

// Squares a number using AsMut. Add the trait bound as is appropriate and
// implement the function body.
fn num_sq(arg: &mut impl AsRef<str>) {
    *arg.as_mut() *= *arg.as_mut()
}
```
~~~

~~~admonish success title="solution3: where clause" collapsible=true
```rust,editable
// Obtain the number of bytes (not characters) in the given argument
// Add the AsRef trait appropriately as a trait bound
fn byte_counter<T>(arg: T) -> usize
where T: AsRef<str>
{
    arg.as_ref().as_bytes().len()
}
```
~~~

### from_into

- The From trait is used for value-to-value conversions.
- If From is implemented correctly for a type, the Into trait should work conversely.
- You can read more about it at: [From in std::convert - Rust](https://doc.rust-lang.org/std/convert/trait.From.html)

> Your task is:

- to complete this implementation in order for the line `let p = Person::from("Mark,20")` to compile
- Please note that you'll need to parse the age component into a `usize` with something like `"4".parse::<usize>()`.
- The outcome of this needs to be handled appropriately.

> Steps:

1. If the length of the provided string is 0, then return the default of Person
2. Split the given string on the commas present in it
3. Extract the first element from the split operation and use it as the name
4. If the name is empty, then return the default of Person
5. Extract the other element from the split operation and parse it into a `usize` as the age
   If while parsing the age, something goes wrong, then return the default of Person
   Otherwise, then return an instantiated Person object with the results

~~~admonish note title="from_into: We implement the Default trait to use it as a fallback when the provided string is not convertible into a Person object" collapsible=true
```rust,editable
{{#include from_into.rs}}
```
~~~

~~~admonish tip title="Hint" collapsible=true
Follow the steps provided right before the `From` implementation
~~~

~~~admonish success title="solution" collapsible=true
```rust,editable
impl From<&str> for Person {
    fn from(s: &str) -> Person {
        match s.split_once(',') {
            Some((first, second)) => {
                if first.is_empty() {
                    Person::default()
                } else if let Ok(a) = second.parse::<usize>() {
                    Person {
                        name: first.into(),
                        age: a,
                    }
                } else {
                    Person::default()
                }
            },
            _ => Person::default(),
        }
    }
}
```
~~~

### from_str

- This is similar to from_into.rs, but this time we'll implement `FromStr` and return errors instead of falling back to a default value.
- Additionally, upon implementing FromStr, you can use the `parse` method on strings to generate an object of the implementor type.
- You can read more about it at [FromStr in std::str - Rust](https://doc.rust-lang.org/std/str/trait.FromStr.html)

> Steps:

1. If the length of the provided string is 0, an error should be returned
2. Split the given string on the commas present in it
3. Only 2 elements should be returned from the split, otherwise return an error
4. Extract the first element from the split operation and use it as the name
5. Extract the other element from the split operation and parse it into a `usize` as the age
   with something like `"4".parse::<usize>()`
6. If while extracting the name and the age something goes wrong, an error should be returned
   If everything goes well, then return a Result of a Person object

As an aside: `Box<dyn Error>` implements `From<&'_ str>`. This means that if you want to return a
string error message, you can do so via just using return `Err("my error message".into())`.

~~~admonish note title="from_str" collapsible=true
```rust,editable
{{#include from_str.rs}}
```
~~~

~~~admonish tip title="Hint" collapsible=true
1. The implementation of FromStr should return an Ok with a Person object,
or an Err with an error if the string is not valid.

This is almost like the `from_into` exercise, but returning errors instead
of falling back to a default value.

Look at the test cases to see which error variants to return.

2. Another hint: You can use the `map_err` method of `Result` with a function
or a closure to wrap the error from `parse::<usize>`.

3. Yet another hint: If you would like to propagate errors by using the `?`
operator in your solution, you might want to look at
[Other uses of ? - The Rust Programming Language](https://kuanhsiaokuo.github.io/the-rust-programming-book-khk/rust_by_example_src/error/multiple_error_types/reenter_question_mark.html)
~~~

~~~admonish failure title="solution1" collapsible=true
```rust,editable
// Using ParsePersonError::{Empty, BadLen, NoName, ParseInt} above
impl FromStr for Person {
    type Err = ParsePersonError;
    fn from_str(s: &str) -> Result<Person, Self::Err> {
        if s.is_empty() {
            Err(Empty) // i.e., Err(ParsePersonError::Empty)
        } else {
            let p: Vec<&str> = s.split(',').collect();
            if p.len() != 2 {
                Err(BadLen)
            } else if p[0].len() == 0 {
                Err(NoName)
            } else { 
                match p[1].parse::<usize>() {
                    Ok(a) => Ok(Person { name: p[0].to_string(), age: a }),
                    Err(a) => Err(ParseInt(a)),
                }
            }
        }
    }
}
```
~~~

~~~admonish success title="solution2: ParsePersonError::{ErrorType}" collapsible=true
```rust,editable
// Using ParsePersonError::{Empty, BadLen, NoName, ParseInt} above
impl FromStr for Person {
    type Err = ParsePersonError;
    fn from_str(s: &str) -> Result<Person, Self::Err> {
        if s.is_empty() {
            Err(ParsePersonError::Empty) // i.e., Err(ParsePersonError::Empty)
        } else {
            let p: Vec<&str> = s.split(',').collect();
            if p.len() != 2 {
                Err(ParsePersonError::BadLen)
            } else if p[0].len() == 0 {
                Err(ParsePersonError::NoName)
            } else { 
                match p[1].parse::<usize>() {
                    Ok(a) => Ok(Person { name: p[0].to_string(), age: a }),
                    Err(a) => Err(ParsePersonError::ParseInt(a)),
                }
            }
        }
    }
}
```
~~~

### try_from_into

- TryFrom is a simple and safe type conversion that may fail in a controlled way under some circumstances.
- Basically, this is the same as From.
- The main difference is that this should return a Result type
  instead of the target type itself.
- You can read more about it at: [TryFrom in std::convert - Rust](https://doc.rust-lang.org/std/convert/trait.TryFrom.html)

~~~admonish note title="try_from_into" collapsible=true
```rust,editable
{{#include try_from_into.rs}}
```
~~~

~~~admonish tip title="Hint" collapsible=true
1. Follow the steps provided right before the `TryFrom` implementation.
You can also use the example at: [TryFrom in std::convert - Rust](https://doc.rust-lang.org/std/convert/trait.TryFrom.html)

Is there an implementation of `TryFrom` in the standard library that
can both do the required integer conversion and check the range of the input?

2. Another hint: Look at the test cases to see which error variants to return.

3. Yet another hint: You can use the `map_err` or `or` methods of `Result` to
convert errors.

4. Yet another hint: If you would like to propagate errors by using the `?`
operator in your solution, you might want to look at:
[Other uses of ? - The Rust Programming Language](https://kuanhsiaokuo.github.io/the-rust-programming-book-khk/rust_by_example_src/error/multiple_error_types/reenter_question_mark.html)

~~~

~~~admonish success title="solution" collapsible=true
```rust,editable
// Tuple implementation
impl TryFrom<(i16, i16, i16)> for Color {
    type Error = IntoColorError;
    fn try_from(tuple: (i16, i16, i16)) -> Result<Self, Self::Error> {
        for i in [tuple.0, tuple.1, tuple.2] {
            if i < 0 || i > 255 {
                return Err(IntoColorError::IntConversion)
            }
        }
        Ok(Color { red: tuple.0 as u8, green: tuple.1 as u8, blue: tuple.2 as u8 })
    }
}

// Array implementation
impl TryFrom<[i16; 3]> for Color {
    type Error = IntoColorError;
    fn try_from(arr: [i16; 3]) -> Result<Self, Self::Error> {
        Self::try_from((arr[0], arr[1], arr[2]))
    }
}

// Slice implementation
impl TryFrom<&[i16]> for Color {
    type Error = IntoColorError;
    fn try_from(slice: &[i16]) -> Result<Self, Self::Error> {
        if slice.len() != 3 {
            Err(IntoColorError::BadLen)
        } else {
            Self::try_from((slice[0], slice[1], slice[2]))
        }
    }
}
```
~~~

~~~admonish danger title="challenge!" collapsible=true
Challenge: Can you make the `TryFrom` implementations generic over many integer types?
~~~

### using_as

- Type casting in Rust is done via the usage of the `as` operator.
- Please note that the `as` operator is not only used when type casting.
- It also helps with renaming imports.

> The goal is to make sure that the division does not fail to compile
> and returns the proper type.

~~~admonish note title="using_as" collapsible=true
```rust,editable
{{#include using_as.rs}}
```
~~~

~~~admonish tip title="Hint" collapsible=true
Use the `as` operator to cast one of the operands in the last line of the
`average` function into the expected return type.
~~~

~~~admonish success title="solution" collapsible=true
```rust, editable
fn average(values: &[f64]) -> f64 {
    let total = values.iter().sum::<f64>();
    total / values.len() as f64
}
```
~~~
