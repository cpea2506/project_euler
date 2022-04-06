// Multiples of 3 or 5

fn sum_divisible_by(target: u32, n: u32) -> u32 {
    let p = (target - 1) / n;

    n * p * (p + 1) / 2
}

fn sum(target: u32) -> u32 {
    sum_divisible_by(target, 3) + sum_divisible_by(target, 5) - sum_divisible_by(target, 15)
}

pj_euler::run!("Multiple of 3 or 5", sum(1000));

pj_euler::test!(
    {sum_divisible_by_3_with_target_10, sum_divisible_by(10, 3), 18},
    {sum_divisible_by_5_with_target_10, sum_divisible_by(10, 5), 5},
    {sum_divisible_by_15_with_target_10, sum_divisible_by(10, 15), 0},
    {sum_below_ten, sum(10), 23},
    {sum_above_ten, sum(20), 78}
);
