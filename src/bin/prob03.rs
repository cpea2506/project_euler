// Largest prime factor

fn largest_prime_factor(mut num: u64) -> u64 {
    while num.is_multiple_of(2) {
        num >>= 1;
    }

    // it must be odd here
    while num.is_multiple_of(3) {
        num /= 3;
    }

    let mut i = 5;
    while i * i < num {
        while num.is_multiple_of(i) {
            num /= i;
        }

        while num.is_multiple_of(i + 2) {
            num /= i + 2;
        }

        i += 6;
    }

    num
}

pj_euler::run!("Largest prime factor", largest_prime_factor(600851475143));

pj_euler::test!(
    larget_prime_factor {
        {largest_prime_factor_of_13195, largest_prime_factor(13195), 29},
        {largest_prime_factor_of_333, largest_prime_factor(333), 37},
        {largest_prime_factor_of_331, largest_prime_factor(331), 331},
    }
);
