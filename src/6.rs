use std::num::Wrapping;

fn main() {
    let num1 = Wrapping(10);
    let num2 = Wrapping(2);
    println!("{}", num1.overflowing_sub(num2).0);
}
