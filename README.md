# Rust Guessing Game

A small command-line guessing game written in Rust.

## What Was Fixed

The project was updated to match the `rand` 0.10 API.

- Replaced the old `rand::thread_rng().gen_range(...)` usage.
- Switched to `rand::rng()` and `random_range(...)`.
- Imported `rand::RngExt` so `random_range` is available on the RNG.

This resolves the compile error caused by using an older `rand` example with the newer crate version in `Cargo.toml`.

## Concepts Learned While Implementing

### 1. Working With External Crates

Rust examples often change across crate versions. In this project, the main lesson was that code written for an older `rand` release does not always compile against a newer one. The fix was to read the compiler error, check the current API, and update the call site instead of forcing the old pattern to work.

### 2. Random Number Generation

The secret number is created with `rand::rng()` and `random_range(1..=100)`. This keeps the game logic simple while still giving each run a different answer.

### 3. Input Parsing and Validation

The program reads a line from standard input, trims whitespace, and attempts to parse it into a `u32`. Invalid input is skipped with `continue`, which keeps the game responsive without crashing.

### 4. Loop-Based Game Flow

The guessing game is built around a loop that keeps asking for input until the user finds the correct number. This is a common Rust pattern for interactive command-line programs.

### 5. Tracking Player Progress

The `guess_count` variable records how many valid guesses the player made before winning. This is a simple example of state management inside a loop.

## Build

```bash
cargo check
```

## Run

```bash
cargo run
```
