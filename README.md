# Safe Vector Access in Rust

This repository demonstrates a common error in Rust when accessing elements in a vector: out-of-bounds indexing.  The example showcases how to safely access vector elements and handle potential errors using the `get()` method and `unwrap()`.

## The Problem

Directly accessing elements using array indexing (`vec[index]`) in Rust can result in a runtime panic if the index is out of bounds. This is a potential source of crashes in your application.

## The Solution

The safer approach is to use the `get()` method, which returns an `Option`. This `Option` will be `Some(value)` if the index is valid and `None` otherwise.  This allows you to handle the case where the index is out of bounds gracefully, avoiding runtime panics.

## How to Run

1. Clone the repository.
2. Navigate to the project directory.
3. Compile and run the code using `cargo run`.
