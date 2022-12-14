# Practice demos for Rust

## Rreferences

The concept *reference* has many faces in Rust. Although sometimes they are explained or modelled by pointers, but they have severals additional features, mostly in form of additional constraints, but sometimes properties not directyl modellable by a pointer.

### Can references appear in constant declarations?

We can build a reference even to a literal, and provide it in constant declarations:

[source code sample](const_refs/src/main.rs)

That is somewhat strange from a pointer, and must be practiced on its own.

### Mimicking references with functions returning ownership received in arguments

references are often explained as a tool to avoid excessively length function signatures and usage, and to prevent the function from devouring the ownerships reveived in its arguments:

[source code sample](move_vs_borrow/src/main.rs)

An interesting practice is if we try to learn renferences in a way that we try to build for any use case of a reference a sample scenario avoiding reference usaga completely and substututing it with using merely the ownership-returning style.
