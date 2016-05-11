extern crate readline;

use std::process::exit;

fn eval(line: Option<String>) {
    let line = match line {
        Some(line) => line,
        None => {
            println!("");
            exit(0);
        }
    };

    if line == "quit" {
        exit(0);
    }

    // add words that start with 'a' to the history to demonstrate
    else if line[0 .. 1] == "a".to_string() {
        readline::add_history(line.as_ref());
    }

    println!("Input: '{}'", line);
}

fn main() {
    readline::rl_callback_handler_install("Next: ", eval);

    // simple r"e"pl
    loop {
        // a real program would interleave this with other async i/o, using
        // something like mio
        readline::rl_callback_read_char();
    }
}
