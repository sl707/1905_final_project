use std::{f32::consts::E, io};

fn main() {
    println!("Welcome to RustChat! Choose one of the options below.");
    println!("  1 - Global Chat");
    println!("  2 - Group Chat");
    println!("  3 - Individual Chat");
    println!("  4 - Settings");
    println!("  5 - Exit");
    println!("Type your option number below: ");
    let mut buffer = String::new();
    match io::stdin().read_line(&mut buffer) {
        Ok(_) => {
            println!("RustChat successfully exited. Come back soon!");
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
    ()
}
