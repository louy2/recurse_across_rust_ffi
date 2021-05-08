# Recurse Across Rust FFI

Just a little repo to try out the first example in the post [Move Constructors in Rust:
Is it possible?](https://mcyoung.xyz/2021/04/26/move-ctors)

## Usage

Clone the repository, install Rust, and

    cargo run

## Surprise

Missing the line end in `write_all` in `into_c.rs` exposes the buffering behavior of writing to stdout.