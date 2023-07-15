// Non-Abundant Sums

fn abundant_numbers(limit: usize) -> Vec<usize> {
    let mut proper_divisors_sum = vec![0; limit];

    for i in 1..limit {
        for d in (i * 2..limit).step_by(i) {
            proper_divisors_sum[d] += i;
        }
    }

    (2..limit)
        .filter(|&n| proper_divisors_sum[n] > n)
        .collect::<Vec<usize>>()
}

fn non_abundant_sums(limit: usize) -> usize {
    let abundant_numbers = abundant_numbers(limit);
    let mut sum_of_2_abundants_sieve = vec![false; limit];

    for (i, &a) in abundant_numbers.iter().enumerate() {
        for b in &abundant_numbers[i..] {
            let sum = a + b;

            if sum >= limit {
                break;
            }

            sum_of_2_abundants_sieve[sum] = true;
        }
    }

    sum_of_2_abundants_sieve
        .iter()
        .enumerate()
        .filter(|&(_, &n)| !n)
        .map(|(i, _)| i)
        .sum()
}

pj_euler::run!("Non-Abundant Sums", non_abundant_sums(28123));

pj_euler::test!(
    non_abundant_sums {
        {smallest_abundant_number, abundant_numbers(16), vec![12]},
    }
);
