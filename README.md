Rust-Graphics
=============

A meta library for computer graphics, written in Rust, that works with multiple backends

Notice! This project is still on the drafting table!

## Motivation

### Easy of use

Inspired by the functional features in Rust standard library,
it would be nice to be able to write something like this:

```Rust
let c = Context::new();  // Using OpenGL.
let d = c.trans(10.0, 10.0);
d.rect(0.0, 0.0, 200.0, 100.0).margin(5).draw();
d.ellipse(0.0, 0.0, 200.0, 100.0).border(3).draw();
let d = c.trans(20.0, 20.0);
...
```

Design:

* Not have to 'undo' changes to the context
* Building context from general to special

### Why the current situation is not acceptable

It takes *hours* to get something basic on the screen with Rust. Existing libraries are limited by the type system and platform specific dependencies.

### Backends

The problem is that a lot of software depend on a graphical user interface, which makes it difficult to port applications. It would be nice to solve this by having one API that worked against customizable backends.

There are relatively few graphics functions required to make pretty advanced games or applications. I would like to write a graphics API in pure Rust that can easily be adapted to multiple backends. This makes it possible to expand the meta library without having to hard code new features per application. With control over the backend there is nothing that prevents you to hard code for special cases.

List of potential backends:

* OpenGL
* OpenGL-ES
* DirectX
* JSON
* JavaScript HTML canvas
* SVG
* PostScript
* Cairo
* Cocoa
* CPU rendering
* ...

## Goals

* To have a feature complete library for 2D graphics in general
* Easy to use
* Vector graphics
* Images
* Clipping
* Text layout
* Minimal dependencies
* Reasonable defaults with resolution detection whenever possible
* Image formats in pure Rust

## Non-Goals

* Backward compability (expect lot of breaking)
* Platform or backend specific code
* 3D
* Physics
* Node tree
* One-to-one correspondence with standards

