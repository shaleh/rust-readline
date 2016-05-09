rust-readline
=============

[![Join the chat at https://gitter.im/shaleh/rust-readline](https://badges.gitter.im/shaleh/rust-readline.svg)](https://gitter.im/shaleh/rust-readline?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)

Simple wrapper around readline for the Rust language

Exposes:
 - `add_history(line: &str)`
 - `history() -> Vec<String>`
 - `history_expand(input: &str) -> Result<Option<String>, String>`
 - `history_is_stifled() -> bool`
 - `stifle_history(n: i32)`
 - `unstifle_history() -> i32`
 - `readline(prompt: &str) -> Option<String>`

A Gitter [channel](https://gitter.im/shaleh/rust-readline) is available.
