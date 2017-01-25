# Lucifer

Lucifer is a physically based renderer, written as a tutorial on how
to implement raytracing and pathtracing in Rust.

Lucifer tries to showcase a very clean and easy-to-understand
implementation of the basics of 3D rendering using raytracing.

The first few [commits](https://github.com/ennocramer/lucifer) are
structured as a tutorial, showing the implementation of the core
structures and algorithms of a [path
tracing](https://en.wikipedia.org/wiki/Path_tracing) renderer.

## Building

Lucifer is written in [Rust](https://www.rust-lang.org/) and may,
depending on when you're trying to build it, require on the nightly
release of the Rust compiler.

``` sh
# Install Rust via Rustup
$ curl https://sh.rustup.rs -sSf | sh

# Install nightly release, if necessary
$ rustup install nightly
$ rustup default nightly

# Build lucifer
$ cargo build --release

# Run lucifer to generate the example output
$ cargo run --release example.png
```
