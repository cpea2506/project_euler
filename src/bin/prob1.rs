// Multiples of 3 or 5

fn sum(bound: u16) -> u16 {
    (3..bound).filter(|n| n % 3 == 0 || n % 5 == 0).sum()
}

pj_euler::solution!("Multiple of 3 or 5", sum(1000));

pj_euler::test!(
    {sum_below_ten, sum(10), 23},
    {sum_above_ten, sum(20), 78}
);
