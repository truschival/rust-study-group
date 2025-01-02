use apples_oranges::{Apple, Orange};

fn main() {
    let apple = Apple::new("Elstar".to_string(), apples_oranges::Color::Red, 0.35);

    let orange_apple = Apple::new(
        "Orange Apple".to_string(),
        apples_oranges::Color::Orange,
        0.65,
    );

    let orange = Orange::new(
        apples_oranges::OrangeType::Valencia,
        0.5,
        apples_oranges::Color::Orange,
    );

    println!("Apple: {}", apple);
    println!("Orange: {:?}", orange);

    println!(" I can compare Apples and Oranges! ");
    println!("Orange < apple: {}", orange < apple);
    println!("Orange > apple: {}", orange > apple);
    println!("Orange == apple: {}", orange == apple);
    println!("Orange == orange_apple: {}", orange != orange_apple);
}
