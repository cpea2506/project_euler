// 10001st prime

use number::prime::IsPrime;

fn nth_prime(bound: u32) -> u32 {
    if bound == 1 {
        return 2;
    }

    let mut count = 1;
    let mut number = 1;

    while count < bound {
        number += 2;

        if number.is_prime() {
            count += 1;
        }
    }

    number
}

pj_euler::run!("10001st prime", nth_prime(10_001));

pj_euler::test!(
    prime_10001st {
        {one_is_not_prime, 1.is_prime(), false},
        {two_is_prime, 2.is_prime(), true},
        {three_is_prime, 3.is_prime(), true},
        {four_is_not_prime, 4.is_prime(), false},
        {sixth_prime_is_thirteen, nth_prime(6), 13},
        {second_prime_is_three, nth_prime(2), 3},
    }
);
