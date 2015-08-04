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

        println!("Input: '{}'", input);
    }
}
