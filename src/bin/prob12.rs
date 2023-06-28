// Highly Divisible Triangular Number

trait Divisor {
    fn divisor_count(self) -> Self;
}

impl Divisor for usize {
    fn divisor_count(mut self) -> Self {
        let mut count = 1;

        for prime in 2usize.. {
            if prime.pow(2) > self {
                count *= 2;
                break;
            }

            if self % prime == 0 {
                let mut factor_count = 0;

                while self % prime == 0 {
                    self /= prime;
                    factor_count += 1;
                }

                count *= factor_count + 1;

                if self == 1 {
                    break;
                }
            }
        }

        count
    }
}

struct TriangularNumber {
    sum: usize,
    curr: usize,
}

impl TriangularNumber {
    fn new() -> Self {
        Self { sum: 0, curr: 1 }
    }
}

impl Iterator for TriangularNumber {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.sum += self.curr;
        self.curr += 1;

        Some(self.sum)
    }
}

fn highly_divisible_triangular_number(num_divisor: usize) -> usize {
    let mut triangular_number = TriangularNumber::new();

    triangular_number
        .find(|v| v.divisor_count() >= num_divisor)
        .unwrap_or_default()
}

pj_euler::run!(
    "Highly Divisible Triangular Number",
    highly_divisible_triangular_number(500)
);

pj_euler::test!(
    highly_divisible_triangular_number {
        {over_5_divisors, highly_divisible_triangular_number(5),  28},
        {divisor_count_of_6_is_4, 6.divisor_count(), 4},
        {divisor_count_of_28_is_6, 28.divisor_count(), 6}
    }
);
