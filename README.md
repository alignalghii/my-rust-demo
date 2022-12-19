# Practice demos for Rust

## Table of contents

- [The concept of references](#the-concept-of-references)
    - [Can references appear in constant declarations?](#can-references-appear-in-constant-declarations)
    - [Mimicking references with functions returning ownership received in arguments](#mimicking-references-with-functions-returning-ownership-received-in-arguments)
    - [Mutable references](#mutable-references)


## The concept of references

The concept *reference* has many faces in Rust. Although sometimes they are explained or modelled by pointers, but they have severals additional features, mostly in form of additional constraints, but sometimes properties not directyl modellable by a pointer.

### Can references appear in constant declarations?

We can build a reference even to a literal, and provide it in constant declarations:

[source code sample](const_refs/src/main.rs)

That is somewhat strange from a pointer, and must be practiced on its own.

### Mimicking references with functions returning ownership received in arguments

references are often explained as a tool to avoid excessively length function signatures and usage, and to prevent the function from devouring the ownerships reveived in its arguments:

[source code sample](move_vs_borrow/src/main.rs)

An interesting practice is if we try to learn renferences in a way that we try to build for any use case of a reference a sample scenario avoiding reference usaga completely and substututing it with using merely the ownership-returning style.

### Mutable references

Mutable references are not an easy concept, partially because it is easy to mix them with a related,  concept.

Let us contrast the following two function declarations:

- `fn inc_ref(nref: &mut i32)`
- `fn print_reseated_ref(mut nref: &i32)`

It is the first one that is an important concept, and it is the second one that is an existing, but marginal and not too useful concept. They are easy to be mixed while a beginner is taking the learning curve, and that can hide the most important essence. The important conceptual difference between them is detailed in the [following sample code file](increment_reference/src/main.rs).

The essence is the conceptual difference between pointer reseating versus contentual refilling.  In the pointer reseating sample, also note the use of a (global) constant: if we change that for a local variable, in most cases we run into interesting lifetime problems.