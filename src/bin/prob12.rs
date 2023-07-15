// Highly Divisible Triangular Number

use sequence::triangular_sequence::TriangularSequence;

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

fn highly_divisible_triangular_number(num_divisor: usize) -> Option<usize> {
    let mut triangular_number = TriangularSequence::<usize>::new();

    triangular_number.find(|v| v.divisor_count() >= num_divisor)
}

pj_euler::run!(
    "Highly Divisible Triangular Number",
    highly_divisible_triangular_number(500)
);

pj_euler::test!(
    highly_divisible_triangular_number {
        {over_5_divisors, highly_divisible_triangular_number(5), Some(28)},
        {divisor_count_of_6_is_4, 6.divisor_count(), 4},
        {divisor_count_of_28_is_6, 28.divisor_count(), 6},
    }
);
