# oo-bindgen

Object-oriented binding generator for Rust.

## How it works

- First, you write your Rust library without thinking about bindings.
- Then, you write a C FFI to your Rust library, taking into account how object-
  oriented languages will interact with it. You also make sure to protect as
  much as possible the interface between your Rust library and the outside C
  world
- You define a general object-oriented "schema" that uses the C FFI to interact
  with your library.
- You generate the bindings in the target languages using generators that reads
  the previously defined "schema" and generate easy-to-use, idiomatic and
  portable code.
- You write unit tests in the generated languages to make sure everything works
  as expected.
- 💲 Profits

## Directories

- `oo-bindgen`: main library to build an object-oriented representation of your
  library.
- `generators`: different generators that takes a library defined using
  `oo-bindgen` to create easy-to-use bindings.
- `tests`: contains an example `foo-ffi` library with the associated
  `foo-bindings` object-oriented library definition. It builds the same library
  in each supported language. Each language has extensive unit tests written to
  check that the generated bindings work as expected.
- `example`: a full real-life example of a library exposing `dnp3rs` to other
  languages.