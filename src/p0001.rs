//! PROBLEM 0001
//! Author: Michael
//! 
//! If we list
//! all the natural numbers
//! below 10 
//! that are multiples of 3 or 5
//! we get 3, 5, 6 and 9.
//! The sum
//! of these multiples
//! is 23.
//! Find the sum
//! of all the multiples of 3 or 5
//! below 1000.

pub fn go() {
    let mut n = 3;
    let mut total = 0;

    loop {
        if n % 3 == 0 || n % 5 == 0 {
            total += n;
        }
        
        n += 1;

        if n >= 1000 {
            break;
        }
    }
    
    println!("P0001 result: {}", total);
}