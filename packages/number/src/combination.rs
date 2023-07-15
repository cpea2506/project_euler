use num_traits::{CheckedAdd, One, Zero};
use std::{
    borrow::BorrowMut,
    ops::{Div, MulAssign, SubAssign},
};

pub struct Combination<T> {
    n: T,
    k: T,
}

impl<T> Combination<T>
where
    T: PartialOrd + SubAssign + One + Copy + CheckedAdd + Zero + MulAssign + Div<Output = T>,
{
    #[inline]
    pub fn new(n: T, k: T) -> Self {
        assert!(n > k, "n must be larger than k");

        Self { n, k }
    }

    /// Get the number of ways to choose an (unordered)
    /// subset of `k` elements from a fixed set of `n` elements
    #[inline]
    pub fn value(self) -> T {
        let mut n = self.n;

        num_iter::range_step_inclusive(One::one(), self.k, One::one())
            .borrow_mut()
            .fold(One::one(), |mut acc, k| {
                acc *= n;
                n -= One::one();

                acc / k
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn c2_4() {
        assert_eq!(Combination::new(4, 2), 6);
    }
}
