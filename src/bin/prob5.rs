// Smallest multiple

use std::ops::RangeInclusive;

use num_integer::Integer;

fn evenly_divisible_by(range: RangeInclusive<u32>) -> Option<u32> {
    range.reduce(|a, b| a.lcm(&b))
}

pj_euler::run!("Smallest multiple", evenly_divisible_by(1..=20));

pj_euler::test!(
    smallest_multiple {
        {one_to_five, evenly_divisible_by(1..=5), Some(60)},
        {one_to_seven, evenly_divisible_by(1..=7), Some(420)},
        {one_to_ten, evenly_divisible_by(1..=10), Some(2520)},
        {one_to_thirteen, evenly_divisible_by(1..=13), Some(360360)}
    }
);
