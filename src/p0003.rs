//! PROBLEM 0003
//! Author: Michael
//! 
//! The prime factors of 13195
//! are 5, 7, 13 and 29.
//! What is the largest prime factor
//! of the number 600851475143 ?

fn is_prime(n: u64) -> bool {
    for i in (3..n).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }

    return true
}

pub fn go() {
    // let n: u64 = 13195;
    let n: u64 = 600851475143;

    let mut multiplied_prime_factors = 1;
    let mut i = 1;

    let largest_prime_factor = loop {
        if n % i == 0 && is_prime(i) {
            multiplied_prime_factors *= i;

            if n == multiplied_prime_factors {
                break i
            };
        }

        i += 2;
    };

    println!("P0003 result: {}", largest_prime_factor);
}