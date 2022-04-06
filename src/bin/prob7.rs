fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }

    if n < 4 {
        return true;
    }

    if n % 2 == 0 {
        return false;
    }

    if n < 9 {
        return true;
    }

    if n % 3 == 0 {
        return false;
    }

    let limit = (n as f64).sqrt() as u32;
    let mut number = 5;

    while number <= limit {
        if n % number == 0 {
            return false;
        }

        if n % (number + 2) == 0 {
            return false;
        }

        number += 6;
    }

    true
}

fn nth_prime(bound: usize) -> u32 {
    if bound == 1 {
        return 2;
    }

    let mut count = 1;
    let mut number = 1;

    while count < bound {
        number += 2;

        if is_prime(number) {
            count += 1;
        }
    }

    number
}

pj_euler::solution!("prob7", nth_prime(10_001));

pj_euler::test!(
    {one_is_not_prine, is_prime(1), false},
    {two_is_prime, is_prime(2), true},
    {three_is_prime, is_prime(3), true},
    {four_is_not_prime, is_prime(4), false},
    {sixth_prime_is_thirteen, nth_prime(6), 13},
    {second_prime_is_three, nth_prime(2), 3}
);
