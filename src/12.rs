use rand::prelude::*;
fn main() {
    let mut rng = thread_rng();
    let x: i32 = rng.gen_range(1..=10);
    let y: i32 = rng.gen_range(1..=10);
    println!("{} + {} = ", x, y);
}
