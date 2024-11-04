use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn strcmp(s1: &str, s3: &str) {
    //! Compare two strings and print the result
    //! args: s1, s2: &str
    match s1.cmp(s3) {
        Ordering::Less => println!("{} < {}", &s1, s3),
        Ordering::Greater => println!("{} > {}", &s1, s3),
        Ordering::Equal => println!("{} == {}", &s1, s3),
    }
}

#[doc = r"another comment"]
pub fn something_else() -> u64 {
    //! This function returns a u64
    34u64
}

/// https://stackoverflow.com/questions/21747136/how-do-i-print-the-type-of-a-variable-in-rust/43508373#43508373
fn print_type_of<T>(_: &T) {
    println!("Type {}", std::any::type_name::<T>());
}

/// Main function
fn main() {
    let r1 = 1..something_else();
    //let r1 = -1..100;
    let str1 = "Guess a number in range";
    println!("{} {:?}!", str1, &r1);

    let str2 = "Hello, world!";
    strcmp(str1, str2);

    // type inference is weird... this type is inferred by match statement !?!?!
    let secret = rand::thread_rng().gen_range(1..100);
    print_type_of(&secret);

    let f = rand::thread_rng().gen_range(r1);

    loop {
        let mut guess = String::new();
        println!("Please input your guess.");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        {
            // shadowing guess
            // parse returns a Result type matching left hand side type
            let guess = match guess.trim().parse::<u64>() {
                // let guess = match guess.trim().parse::<u64>() {
                // let guess: i32 = match guess.trim().parse() {
                Ok(peter) => peter + 100,
                Err(_) => {
                    println!("Error: cannot parse '{}' as a number!", guess.trim());
                    10000
                }
            };
            println!("You guessed: {}", guess);

            println!("Type of guess: (should be u64)");
            print_type_of(&guess);

            //match guess.cmp(&secret) {
            match guess.cmp(&secret) {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal => {
                    println!("You win!");
                    break;
                }
            }
        }
        println!("Type of guess should be string");
        print_type_of(&guess);
    }
}
