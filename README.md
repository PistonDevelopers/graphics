Rust-Graphics
=============

A meta library for computer graphics, written in Rust, that works with multiple backends

Notice! This project is still on the drafting table!

## Motivation

There are relatively few graphics functions required to make pretty advanced games or applications. I would like to write a graphics API in pure Rust that can easily be adapted to multiple backends. This makes it possible to expand the meta library without having to hard code new features per application. There are certain limitations to pure libraries, but because it allows you to control the backend there is nothing that prevents you to hard code for special cases.

## Taking advantage of Rust's type system

Computer graphics is a complicated subject, but there are certain things that are universal:

* You want to change the context dynamically
* You want to undo recently changes to the context
* You want to know what is going on in the context
* You don't want to keep track of changes required to undo changes
* You want to check for hit test and get context relevant information
* You want to work with context specific methods to build patterns etc.
* You want good defaults

The `enum` in Rust allows one to create a building block for both values and pointer to values:

```Rust
/// A structure that might contain a value or a borrowed value.
/// This is to used as building block to create data structure
/// that is partially based on an existing structure.
pub enum Maybe<'a, T> {
    /// Contains a value.
    Value(T),
    /// Contains a borrowed pointer.
    Borrowed(&'a T),
}

impl<'a, T> Maybe<'a, T> {
    /// Gets a read only value.
    #[inline(always)]
    pub fn get(&'a self) -> &'a T {
        match *self {
            Value(ref val) => val,
            Borrowed(rval) => rval,
        }
    }
}
```
