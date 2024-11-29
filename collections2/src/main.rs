use collections2;
use std::io;

pub fn main() {
    println!("Enter text to piglify, end with enter!");

    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read input");

    let len = user_input.len();

    for s in user_input[..len - 1].split(" ") {
        if let Some(result) = collections2::piglify_word(&s) {
            print!("{} ", result);
        }
    }
    print!("\n");
}
