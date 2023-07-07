// Amicable Numbers

type Number = usize;

trait Math {
    fn sqrt(self) -> Self;
    fn divisors_sum(self) -> Number;
    fn is_amicable_number(&self) -> bool;
}

impl Math for Number {
    fn sqrt(self) -> Self {
        (self as f64).sqrt() as usize
    }

    fn divisors_sum(self) -> Number {
        (2..self.sqrt()).fold(1, |mut acc, v| {
            if self % v == 0 {
                acc += v;

                let another_divisor = self / v;

                if another_divisor != self {
                    acc += another_divisor;
                }
            }

            acc
        })
    }

    fn is_amicable_number(&self) -> bool {
        let self_divisors_sum = self.divisors_sum();

        self_divisors_sum != *self && self_divisors_sum.divisors_sum() == *self
    }
}

fn sum_amicable_numbers(limit: Number) -> Number {
    (1..limit).filter(Number::is_amicable_number).sum()
}

pj_euler::run!("Amicable Numbers", sum_amicable_numbers(10000));
pj_euler::test!(
    amicable_numbers {
        {amicable_numbers_include_220, 220.is_amicable_number(), true},
        {amicable_numbers_include_284, 284.is_amicable_number(), true},
        {amicable_numbers_do_not_include_2, 2.is_amicable_number(), false}
    }
);
