// 1000-digit Fibonacci Number

use num_bigint::BigUint;
use sequence::fibonacci::Fibonacci;

fn thousand_digit_fibonacci_number(number_of_digit: usize) -> usize {
    let limit = "9".repeat(number_of_digit - 1).parse::<BigUint>().unwrap();

    Fibonacci::<BigUint>::new()
        .take_while(|n| *n <= limit)
        .count()
        + 1
}

pj_euler::run!(
    "1000-digit Fibonacci Number",
    thousand_digit_fibonacci_number(1000)
);

pj_euler::test!(
    thousand_digit_fibonacci_number {
        first_three_digit_has_144 {
            assert_eq!(thousand_digit_fibonacci_number(3), 12);
        },
    }
);
