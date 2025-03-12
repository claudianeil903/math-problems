fn main() {
    let mut rng = rand::thread_rng();
    let num1: i32 = rng.gen_range(0..10);
    let num2: i32 = rng.gen_range(0..10);
    println!("{} + {}", num1, num2);
}
