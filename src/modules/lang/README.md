# Hulang

Hulang is a DSL written in Rust for the `Hulaak` runtime, enabling data
transformations. It is a very simple lisp dialect with some inbuilt functions
and methods. Hulang compiles all expressions to a vector of function calls
internally which get triggered on the source `Message` struct when this module
is used. The mutated `Message` struct is then passed to downstream components.
