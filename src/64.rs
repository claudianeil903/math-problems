fn main() {
    // Exercise 1: Summing natural numbers
    let sum = (0..100).sum();
    println!("Sum of natural numbers from 0 to 100: {}", sum);

    // Exercise 2: Finding the highest power of 3 that divides a number
    let num = 48;
    let highest_power_of_3 = if num % 3 == 0 {
        (num / 3).ceil() as u32
    } else {
        1
    };
    println!("The highest power of 3 dividing {} is: {}", num, highest_power_of_3);

    // Exercise 3: Factorizing a number
    let num = 90;
    let factors = prime_factors(num);
    for &factor in factors.iter() {
        if factor * (factor + 1) == num {
            println!("The divisors of {} are {}", num, factor, factor + 1);
        }
    }

    // Exercise 4: Summing all even numbers from 0 to 50
    let sum_even = (0..50).filter(|&x| x % 2 == 0).sum();
    println!("Sum of even numbers from 0 to 50: {}", sum_even);

    // Exercise 5: Finding the smallest prime factor
    fn is_prime(n: u32) -> bool {
        if n <= 1 {
            return false;
        }
        for i in 2..=n.sqrt() {
            if n % i == 0 {
                return false;
            }
        }
        true
    }

    let number = 45;
    let smallest_prime_factor = is_prime(number).then(|| (number, 1));
    println!("The smallest prime factor of {} is {}", number, smallest_prime_factor.1);
}
