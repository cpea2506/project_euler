// 10001st prime

trait Prime {
    fn is_prime(&self) -> bool;
}

impl Prime for u32 {
    fn is_prime(&self) -> bool {
        let n = *self;

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

        let limit = (n as f32).sqrt() as u32;
        let mut number = 5;

        while number <= limit {
            if self % number == 0 {
                return false;
            }

            if self % (number + 2) == 0 {
                return false;
            }

            number += 6;
        }

        true
    }
}

fn nth_prime(bound: usize) -> u32 {
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
        {one_is_not_prine, 1.is_prime(), false},
        {two_is_prime, 2.is_prime(), true},
        {three_is_prime, 3.is_prime(), true},
        {four_is_not_prime, 4.is_prime(), false},
        {sixth_prime_is_thirteen, nth_prime(6), 13},
        {second_prime_is_three, nth_prime(2), 3}
    }
);
