fn main() {
    // Example Rust code: solving the system of linear equations
    let x = 3.5;
    let y = 2.0;
    let a = 4.0;
    let b = 1.0;

    if x + y == a && x - y == b {
        println!("The solution is x = {}, y = {}", x, y);
    } else {
        println!("There's no solution for the given values.");
    }
}
