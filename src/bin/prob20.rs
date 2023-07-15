// Factorial Digit Sum

use num_bigint::BigUint;
use num_traits::one;

fn factorial_digit_sum(number: u32) -> u32 {
    (2..=number)
        .fold(one::<BigUint>(), |acc, n| acc * n)
        .to_string()
        .chars()
        .filter_map(|n| n.to_digit(10))
        .sum()
}

pj_euler::run!("Factorial Digit Sum", factorial_digit_sum(100));

pj_euler::test!(
    factorial_digit_sum {
        {factorial_digit_sum_with_number_10, factorial_digit_sum(10), 27},
        {factorial_digit_sum_with_number_50, factorial_digit_sum(50), 216},
    }
);
