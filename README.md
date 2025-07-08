
minigrep
========

A simple command-line tool written in Rust that searches for a query string in a file — similar to the Unix `grep` command.

About
-----

This project is a part of The Rust Programming Language Book (Chapter 12) and serves as an introduction to:

- Reading command-line arguments
- Handling file I/O
- Working with iterators
- Error handling with `Result`
- Writing tests and modular code

Usage
-----

    cargo run -- <query> <filename>

Example:

    cargo run -- to poem.txt

For case-insensitive search, set the environment variable:

    CASE_INSENSITIVE=1 cargo run -- to poem.txt

Build & Test
------------

    cargo build
    cargo test

Structure
---------

- src/main.rs — Entry point, handles CLI logic
- src/lib.rs — Core functionality (search, config, etc.)
