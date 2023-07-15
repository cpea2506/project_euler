// Power Digit Sum

use num_bigint::ToBigUint;
use num_traits::pow;

fn power_digit_sum(exponent: usize) -> Option<u32> {
    Some(
        pow(2.to_biguint()?, exponent)
            .to_string()
            .chars()
            .filter_map(|v| v.to_digit(10))
            .sum(),
    )
}

pj_euler::run!("Power Digit Sum", power_digit_sum(1000));

pj_euler::test!(
    power_digit_sum {
        {power_digit_sum_with_exponent_15, power_digit_sum(15), Some(26)},
    }
);
