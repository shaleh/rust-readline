extern crate readline;

fn main() {
    loop {
        let input = match readline::readline("Next: ") {
            Some(input) => input,
            None => {
                println!("");
                break;
            },
        };

        if input == "quit" {
            break;
        }
        // add words that start with 'a' to the history to demonstrate
        else if input[0 .. 1] == "a".to_string() {
            readline::add_history(input.as_ref());
        }

        println!("Input: '{}'", input);
    }
}
