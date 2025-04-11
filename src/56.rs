use std::io;

fn main() {
    println!("Enter the first number:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let num1: isize = input.trim().parse().expect("Invalid input");

    println!("Enter the second number:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let num2: isize = input.trim().parse().expect("Invalid input");

    if num1 == num2 {
        println!("The numbers are equal.");
    } else if num1 > num2 {
        println!("{}", "The first number is greater than the second number.");
    } else {
        println!("{}", "The second number is greater than the first number.");
    }
}
