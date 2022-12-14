//! PROBLEM 0002
//! Author: Michael
//! 
//! Each new term
//! in the Fibonacci sequence is generated
//! by adding the previous two terms.
//! By starting with 1 and 2,
//! the first 10 terms will be:
//! 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...
//! By considering the terms in the Fibonacci sequence
//! whose values do not exceed four million,
//! find the sum of the even-valued terms.

fn main() {
    let mut prev  = 1;
    let mut fib   = 2;
    let mut total = 0;

    loop {
        if fib % 2 == 0 {
            total += fib;
        }

        (prev, fib) = (fib, fib + prev);

        if fib >= 4000000 {
            break;
        }
    }

    println!("Result: {}", total);
}