use core::f32;
use rand::Rng;
use std::cmp::Ordering;

/// https://stackoverflow.com/questions/21747136/how-do-i-print-the-type-of-a-variable-in-rust/43508373#43508373
fn print_type_of<T>(_: &T) {
    println!("Type {}", std::any::type_name::<T>());
}

#[derive(Debug)]
enum MyEnum {
    Foo,
    Bar { x: i32, y: String },
    Baz(f32, i32, u64),
}

impl MyEnum {
    fn do_something(&self) {
        match self {
            MyEnum::Foo => {
                println!("just Foo with no values! ");
            }

            // o.k. wie kann ich das ganze self und nicht nur die elemente bekommen?
            MyEnum::Bar { x, y } => {
                println!("a Bar with x={x} and y={y}");
                println!("self is -- {:?}", self);
                print_type_of(self); // o.k. self ist hier im scope
                                     // Kann ich dann die properties referenzieren?
                                     // println!("self.x = {}", { self.x });
            }
            MyEnum::Baz(a, b, c) => {
                println!("a Baz with {a}, {b} and {c}")
            }
        }
    }

    fn fun(&self) {
        print_type_of(self);
    }
}

fn get_some(guess: i32) -> Option<f64> {
    let secret = rand::thread_rng().gen_range(1..20);
    println!("  guess: {} - secret: {}", &guess, &secret);
    match guess.cmp(&secret) {
        Ordering::Less => None,
        Ordering::Equal => Some(2.0 * (guess as f64)),
        Ordering::Greater => Some(0.8 * (guess as f64)),
    }
}

// fn get_random_myenum () -> Option<MyEnum>{
// 	let rnd = rand::thread_rng().get_range(1..3);
// 	let f : i32;
// }

fn main() {
    let simple_foo = MyEnum::Foo;
    let bar = MyEnum::Bar {
        x: 2,
        y: String::from("barstring"),
    };
    let some_baz = MyEnum::Baz(f32::consts::TAU, 7, 23);

    simple_foo.fun();

    simple_foo.do_something();
    bar.do_something();
    some_baz.do_something();

    for i in [1, 7, 2, 15, 20] {
        let x = get_some(i);
        let s = x;
        match s {
            None => println!("you got nothing"),
            Some(gain) => println!("you bet {i} and got {gain}"),
        }
    }

    let val: i32 = rand::thread_rng().gen_range(10..20);
    if let Some(gain) = get_some(val) {
        println!("this line is likelier to appear for higher values of val ({val}) {gain}")
    };

    // Note the reference ----------------------------+ !
    // String  value partially moved here
    if let MyEnum::Bar { x, y } = &some_baz {
        println!("x: {x}, y: {y}");
    } else {
        println!("not a Bar");
    }

    if let MyEnum::Baz(a, b, c) = some_baz {
        print!("got a baz {a} {b} {c}")
    } else {
        println!("not a baz")
    }

    // Äquivalent, match als expression kann partial move haben.
    let var = bar;
    match &var {
        MyEnum::Bar { x, y } => {
            println!("matched {x} {y}")
        }
        _ => {}
    }
    println!("{:?}", var);
}
