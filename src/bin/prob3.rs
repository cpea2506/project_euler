// Largest prime factor

fn largest_prime_factor(num: u64) -> u64 {
    let mut num = num;
    let mut max_prime = Default::default();

    while num % 2 == 0 {
        num >>= 1;
    }

    // it must be odd here
    while num % 3 == 0 {
        num /= 3;
    }

    let mut i = 5;
    while i * i < num {
        while num % i == 0 {
            max_prime = i;
            num /= i;
        }

        // difference of two prime factors must
        // be at least 2 (except 2 and 3)
        while num % (i + 2) == 0 {
            max_prime = i + 2;
            num /= i + 2;
        }

        i += 6;
    }

    // if the last prime factor is not 2 and 3. Take it!
    if num > 4 {
        max_prime = num;
    }

    max_prime
}

pj_euler::solution!("Largest prime factor", largest_prime_factor(600851475143));

pj_euler::test!({
    largest_prime_factor_of_13195, largest_prime_factor(13195), 29
});
