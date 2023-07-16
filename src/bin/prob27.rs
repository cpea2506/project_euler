// Quadratic Primes

use number::prime::{IsPrime, PrimeSieve};

fn largest_quadratic_primes_product(limit_a: i32, limit_b: i32) -> i32 {
    let mut limit_a = limit_a;
    let mut limit_b = limit_b;

    if limit_a < 0 {
        limit_a *= -1;
    }

    if limit_b < 0 {
        limit_b *= -1;
    }

    let prime_sieve = PrimeSieve::new(limit_b as usize);

    let mut count = 0;
    let mut product = 1;

    for b in prime_sieve.get_primes() {
        for a in -limit_a..limit_a {
            let mut n = 0;

            while (n * n + a * n + b as i32).is_prime() {
                n += 1;
            }

            if count <= n {
                count = n;
                product = a * b as i32;
            }
        }
    }

    product
}

pj_euler::run!(
    "Quadratic Primes",
    largest_quadratic_primes_product(-1000, -1000)
);
