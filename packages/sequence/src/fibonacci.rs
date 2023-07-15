use num_traits::{One, Zero};
use std::ops::{Add, AddAssign, Sub, SubAssign};

pub struct Fibonacci<T> {
    curr: T,
    next: T,
}

impl<T> Default for Fibonacci<T>
where
    T: One + Zero,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Zero + One> Fibonacci<T> {
    #[inline]
    pub fn new() -> Self {
        Fibonacci {
            curr: Zero::zero(),
            next: One::one(),
        }
    }
}

impl<T> Iterator for Fibonacci<T>
where
    T: AddAssign + SubAssign + Add<Output = T> + Sub<Output = T> + Copy,
{
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.curr += self.next;
        self.next = self.curr - self.next;

        Some(self.curr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fibonacci() {
        assert_eq!(
            Fibonacci::new().take(13).collect::<Vec<i32>>(),
            &[1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233],
        );
    }
}
