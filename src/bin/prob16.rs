// Power Digit Sum

fn power_digit_sum(exponent: usize) -> usize {
    let mut digits = vec![2];

    (1..exponent).for_each(|_| {
        let mut temp_digits = vec![0; digits.len() + 1];

        for (digit, temp_digit) in digits.iter().zip(temp_digits.iter_mut()) {
            *temp_digit = digit * 2;
        }

        digits.resize(temp_digits.len(), 0);

        let mut borrow = 0;

        for (digit, temp_digit) in digits.iter_mut().zip(temp_digits.iter_mut()) {
            *temp_digit += borrow;
            borrow = *temp_digit / 10;
            *temp_digit %= 10;
            *digit = *temp_digit;
        }

        //PERF: remove the trailing zeros to avoid
        // reallocating too much memory
        if let Some(index) = digits.iter().rposition(|&n| n != 0) {
            digits.truncate(index + 1);
        }
    });

    digits.iter().sum()
}

pj_euler::run!("Power Digit Sum", power_digit_sum(1000));

pj_euler::test!(
    power_digit_sum {
        {power_digit_sum_with_exponent_15, power_digit_sum(15), 26}
    }
);
