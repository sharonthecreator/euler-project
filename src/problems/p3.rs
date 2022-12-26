// finds if the number is prime.
fn is_prime(number: u64) -> bool {
    let factor_limit = (number as f64).sqrt().floor() as u64;

    for factor in 2..factor_limit {
        if number % factor == 0 {
            return false;
        }
    }
    return true;
}

pub fn solution() {
    // try compile without explicit typing.
    let number: u64 = 600851475143;
    let factor_limit = (number as f64).sqrt().floor() as u64;
    let mut factor = factor_limit;

    let largest_factor = loop {
        if is_prime(factor) && number % factor == 0 {
            // found the largest prime factor.
            break factor;
        } else if factor <= 1 {
            break 1;
        }

        factor -= 1;
    };

    println!("The largest prime factor of the number {number} is: {largest_factor}");
}