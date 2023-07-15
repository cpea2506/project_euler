// Amicable Numbers

fn sum_amicable_numbers(limit: usize) -> usize {
    let mut divisors_sum = vec![0; limit];

    for i in 1..limit {
        for d in (i * 2..limit).step_by(i) {
            divisors_sum[d] += i;
        }
    }

    divisors_sum
        .iter()
        .enumerate()
        .filter(|&(a, &b)| a != b && a < b && b < limit && divisors_sum[b] == a)
        .fold(0, |acc, (a, b)| acc + a + b)
}

pj_euler::run!("Amicable Numbers", sum_amicable_numbers(10000));

pj_euler::test!(
    amicable_numbers {
        {sum_amicable_numbers_less_than_1000, sum_amicable_numbers(1000), 220 + 284},
    }
);
