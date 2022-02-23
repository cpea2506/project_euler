// Smallest multiple

use std::{mem, ops::RangeInclusive};

// ah ye it's least common multiple. Fine!
// take me almost a day to recognize.

// Stein's algorithm
fn binary_gcd(mut a: u64, mut b: u64) -> u64 {
    if a == 0 {
        return a;
    }

    if b == 0 {
        return b;
    }

    let shift = (a | b).trailing_zeros();

    a >>= shift;
    b >>= shift;
    a >>= a.trailing_zeros();

    while b != 0 {
        b >>= b.trailing_zeros();

        if a > b {
            mem::swap(&mut a, &mut b);
        }

        b -= a;
    }

    a << shift
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / binary_gcd(a, b)
}

fn evenly_divisible_by(range: RangeInclusive<u64>) -> u64 {
    range.reduce(lcm).unwrap_or_default()
}

pj_euler::solution!("Smallest multiple", evenly_divisible_by(1..=20));

pj_euler::test!(
    {one_to_five, evenly_divisible_by(1..=5), 60},
    {one_to_seven, evenly_divisible_by(1..=7), 420},
    {one_to_ten, evenly_divisible_by(1..=10), 2520},
    {one_to_thirteen, evenly_divisible_by(1..=13), 360360}
);
