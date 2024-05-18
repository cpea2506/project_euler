pub trait Sum {
    fn arithmetic_sum(self) -> Self;
    fn arithmetic_sum_with_difference(self, first_term: Self, d: Self) -> Self;
    fn sum_of_squares(self) -> Self;
}

#[macro_export]
macro_rules! impl_sum_for {
    ($($type:ty)+) => {
        $(
            impl Sum for $type {
                fn arithmetic_sum(self) -> $type {
                    self * (self + 1) / 2
                }

                fn arithmetic_sum_with_difference(self, first_term: $type, d: $type) -> $type {
                    self * (2 * first_term + (self - 1) * d) / 2
                }

                fn sum_of_squares(self) -> $type {
                    self * (self + 1) * (2 * self + 1) / 6
                }
            }
        )+
    }
}

impl_sum_for!(usize u32 u64);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn arithmetic_sum_of_10_consecutive_number_with_difference_4_start_with_2() {
        assert_eq!(10usize.arithmetic_sum_with_difference(2, 4), 200);
    }

    #[test]
    fn arithmetic_sum_of_10_consecutive_number() {
        assert_eq!(10usize.arithmetic_sum(), 55);
    }

    #[test]
    fn sum_of_squares_of_10_consecutive_number() {
        assert_eq!(10usize.sum_of_squares(), 385);
    }
}
