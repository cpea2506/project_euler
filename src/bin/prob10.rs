// Summation of primes

fn sum_prime_below(limit: usize) -> usize {
    let sievebound = (limit - 1) / 2;
    let mut sieve = vec![false; sievebound];

    let cross_limit = ((limit as f64).sqrt() as usize - 1) / 2;
    let mut sum = 2;

    for i in 1..cross_limit {
        if !sieve[i] {
            for j in (2 * i * (i + 1)..sievebound).step_by(2 * i + 1) {
                sieve[j] = true
            }
        }
    }

    (1..sievebound).for_each(|i| {
        if !sieve[i] {
            sum += 2 * i + 1;
        }
    });

    sum
}

pj_euler::run!("Summation of primes", sum_prime_below(2_000_000));

pj_euler::test!(
    summation_of_primes {
        {sum_prime_below_10, sum_prime_below(10), 17}
    }
);
