  use std::fmt;
  
  fn main() {
    let mut rng = rand::thread_rng();
  
    // Generate a random number between 1 and 10
    let num1: u32 = rng.gen_range(1..=10);
  
    // Generate another random number between 1 and 10
    let num2: u32 = rng.gen_range(1..=10);
  
    println!("What is {} plus {}?", num1, num2);
  }
  