# xed

Rust bindings for [Intel XED](https://intelxed.github.io/), it's an experimental prototype base on the [xed-sys](https://github.com/rust-xed/xed-sys) crate.

## Building

In order to build this crate, you need:

- Python version 3.8 or later ([to build XED](https://intelxed.github.io/build-manual/)).
- A C compiler.

## Examples

You can find usage examples in the [examples](examples) directory.

These examples are meant to be executed with cargo. For instance, to run the example named `xed-min`:

```sh
# cd to the crate's root directory
$ cargo run --example xed-min
```
