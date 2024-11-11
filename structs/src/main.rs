use std::f32::consts::PI;

#[derive(Debug)] // derived trait, bin gespannt auf das kapitel
struct User {
    is_active: bool,
    name: String,
    email: String,
    login_count: u32,
    // cannot create a ref to other users?
    // how can I create linked lists
    // prev: &User, // refs cannot be null...
    // next: &User,
    // next: Option<User>,
    // neighbors: Vec<&User>,
}

impl User {
    fn create(name: String, email: String) -> Self {
        User {
            name,
            email,
            is_active: false,
            login_count: 0,
        }
    }

    fn create_copy(rhs: &Self) -> Self {
        Self {
            name: rhs.name.clone(),
            email: rhs.email.clone(),
            is_active: rhs.is_active,
            login_count: rhs.login_count,
        }
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn set_name(&mut self, newname: String) {
        self.name = newname;
    }
}

fn main() {
    let thomas = User {
        name: String::from("Thomas"),
        email: String::from("thomas@ruschival.de"),
        is_active: true,
        login_count: 5,
    };

    let some_user = User::create(String::from("otto"), String::from("a@b.com"));
    println!("Hello, \n{:?}", some_user);

    let thomas2 = User {
        is_active: false,
        email: String::from("foo@bar.com"),
        ..thomas
    };

    println!("Hello, world!\n{:?} {}", thomas.email, thomas.login_count); // WOW - it is really moved!

    println!("Hello, world!\n{:?}", thomas2);

    let mut thomas3 = User::create_copy(&thomas2);

    println!("Hello, world!\n{:?}\n{:?}", thomas2, thomas3);

    thomas3.set_name(String::from("Foo"));
    println!("{}", thomas3.name());
}

//------------------------------------------------------------------------------
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// struct Circle {
//     radius: u32,
// }

// impl Rectangle {
//     fn area(r: &Self) -> u32 {
//         r.width * r.height
//     }
// }

// impl Circle {
//     fn area(c: &Self) -> f32 {
//         (c.radius as f32) * 2f32 * PI
//     }
// }
