use errorhandling::{self, return_err_for_15};
use std::fs::OpenOptions;
use std::io::Read;
use std::{fs::File, io::Write};

fn main() {
    let x = return_err_for_15(&3).unwrap();
    println!("x is {}", x.get_value());

    // let _y = return_err_for_15(&15).expect("Should not pass 15!");

    match File::create("./foo.txt") {
        Ok(mut fh) => {
            if let Ok(len) = fh.write(b"hallo Welt!") {
                println!("{len} bytes written!");
            } else {
                panic!("cannot write to file");
            }
        }
        Err(e) => {
            println!("Error {:?}", e);
        }
    }

    let file = OpenOptions::new().read(true).open("./bar.txt");
    let mut content = String::new();
    if let Ok(mut fh) = file {
        match fh.read_to_string(&mut content) {
            Ok(len) => println!("Read {len} bytes -> {}", &content),
            Err(e) => println!("Error reading from file occurred {:?}", e),
        }
    } else {
        println!("Error for open {:?}", file);
    }
}
