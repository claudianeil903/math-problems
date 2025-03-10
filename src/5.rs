use std::rand::{self, Rng};

fn main() {
    let mut rng = rand::thread_rng();

    // Generate a random number between 1 and 10
    let random_number: u32 = rng.gen_range(1..=10);

    println!("The generated number is {}", random_number);
}
