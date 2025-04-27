use std::io;

fn main() {
    let input = String::new();
    println!("Enter a string:");
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let mut chars: Vec<char> = input.chars().collect();

    for (i, c) in chars.iter_mut().enumerate() {
        match i {
            0 => println!("{}", *c),
            _ => (),
        }
    }

    println!("All characters have been printed.");
}
