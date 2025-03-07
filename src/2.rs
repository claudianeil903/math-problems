use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let random_number1: u32 = rng.gen_range(0, 9);
    let random_number2: u32 = rng.gen_range(0, 9);
    
    match random_number1.cmp(&random_number2) {
        Ordering::Less => println!("{} is less than {}", random_number1, random_number2),
        Ordering::Greater => println!("{} is greater than {}", random_number1, random_number2),
        Ordering::Equal => println!("{} is equal to {}", random_number1, random_number2)
    }
}
