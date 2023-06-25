// Highly Divisible Triangular Number

trait Divisor {
    fn divisor_count(self) -> usize;
}

impl Divisor for usize {
    //TODO: use prime factors for faster counting time
    fn divisor_count(self) -> usize {
        if self == 1 {
            1
        } else {
            (1..=self).filter(|v| self % v == 0).count()
        }
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
    { over_5_divisors, highly_divisible_triangular_number(5),  28},
    {divisor_count_of_6_is_4, 3.divisor_count(), 2},
    {divisor_count_of_28_is_6, 28.divisor_count(), 6}
);
