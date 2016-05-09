rust-readline
=============

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
