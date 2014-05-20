Rust-Graphics
=============

A library for 2D graphics, written in Rust, that works with multiple back-ends

[Rust-Graphics online docs](http://bvssvni.github.io/docs/rust-graphics/graphics/)

*Notice: This is a very early stage of the project!*

[How to contribute](https://github.com/PistonDevelopers/rust-graphics/issues/277)

*Last news: Bevel and square lines!*

Example back-end for OpenGL (game engine): [Piston](https://github.com/PistonDevelopers/piston)  
Experimental algorithms are developed in a separate repository: [Rust-Graphics-Lab](https://github.com/PistonDevelopers/rust-graphics-lab)  

## Motivation

### Sharing graphics source code across projects in Rust

Rust is programming language developed by Mozilla and the Rust community. It is fast, safe, concurrent and cross platform. Because of the many numbers of potential platforms (read: all kinds of computers), it would be nice to have a 2d graphics library that works with multiple back-ends, so you don't have to invent a new graphics engine for each platform you are working on.

### Easy of use

Inspired by the functional features in Rust standard library,
it would be nice to be able to write something like this:

```Rust
let mut gl = piston::Gl::new(); // Piston game engine implements BackEnd trait for OpenGL.
let c = Context::new(); // Back-end independent context.
let d = c.trans(10.0, 10.0);
d.rect(0.0, 0.0, 200.0, 100.0).margin(5).fill(&mut gl);
d.ellipse(0.0, 0.0, 200.0, 100.0).border(3).stroke(&mut gl);
let d = c.trans(20.0, 20.0);
...
```

### Performance

The `Context` is immutable and designed to "grow" into other contexts. Internally it uses a borrowed/value field mechanism with lots of inlining to take advantage of compiler optimizations. Usually the data sits on the stack and requires no heap allocations. When a context is "grown" into another through method calls, it copies borrowed pointers for all the unchanged data and allocates new data on the stack. If optimized properly, this only allocates new data on the stack without needing calls to "restore" the context like in other graphics API.

This library is pure Rust, so it will not contain code for a specific back-end. However, it makes assumptions favoring GPU shader rendering. Rust-Graphics looks for supported methods in the order of expected best performance and memory usage.  

### Expressiveness

This design also allows better expressiveness, because one can have context specific methods that are only allowed for some types of contexts, but not all. The library uses traits to provide maximum flexibility and share default behavior across context types. 

### One trait for all back-ends

To write your own back-end `BackEnd` trait. The `BackEnd` trait implements default behavior for all methods, so it is up to you how much code to write. If it can not find a method, it will call the `unimplemented!` macro.  

### Design

* Not have to 'undo' changes to the context
* Building context from general to special
* Helpful context specific methods

## Goals

* To have a feature complete library for 2D graphics in general
* Easy to use
* Vector graphics
* Images (back-end supported)
* Clipping
* Minimal dependencies
* Reasonable defaults with resolution detection whenever possible

## Non-Goals

* Image formats
* Text layout
* Backward compability (expect lot of breaking)
* Platform or back-end specific code
* 3D
* Physics
* Node tree
* One-to-one correspondence with standards
* Integration with platform GUI

