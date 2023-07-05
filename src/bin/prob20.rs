// Factorial Digit Sum

fn factorial_digit_sum(number: usize) -> usize {
    let mut digits = vec![1];

    (2..=number).for_each(|v| {
        let mut temp_digits = vec![0; digits.len() + 1];

        for (digit, temp_digit) in digits.iter().zip(temp_digits.iter_mut()) {
            *temp_digit = digit * v;
        }

        digits.resize(temp_digits.len(), 0);

        let mut borrow = 0;

        for (digit, temp_digit) in digits.iter_mut().zip(temp_digits.iter_mut()) {
            *temp_digit += borrow;
            borrow = *temp_digit / 10;
            *temp_digit %= 10;
            *digit = *temp_digit;
        }

        // borrow can be larger than zero
        digits.push(borrow);

        //PERF: remove the trailing zeros to avoid
        // reallocating too much memory
        if let Some(index) = digits.iter().rposition(|&n| n != 0) {
            digits.truncate(index + 1);
        }
    });

    digits.iter().sum()
}

pj_euler::run!("Factorial Digit Sum", factorial_digit_sum(100));

pj_euler::test!(
    factorial_digit_sum {
        {factorial_digit_sum_with_number_10, factorial_digit_sum(10), 27},
        {factorial_digit_sum_with_number_50, factorial_digit_sum(50), 216}
    }
);
