use errorhandling::{self, return_err_for_15};

fn main() {
    let x = return_err_for_15(&3).unwrap();
    println!("x is {}", x.get_value());

    let _y = return_err_for_15(&15).expect("Should not pass 15!");
}
