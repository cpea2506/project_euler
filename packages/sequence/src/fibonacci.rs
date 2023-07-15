use num_traits::One;
use std::{
    mem,
    ops::{Add, AddAssign, Sub, SubAssign},
};

pub struct Fibonacci<T> {
    curr: T,
    next: T,
}

impl<T> Default for Fibonacci<T>
where
    T: One,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T: One> Fibonacci<T> {
    #[inline]
    pub fn new() -> Self {
        Fibonacci {
            curr: One::one(),
            next: One::one(),
        }
    }
}

impl<T> Iterator for Fibonacci<T>
where
    T: AddAssign + SubAssign + Add<Output = T> + Sub<Output = T> + Clone,
{
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let next = self.curr.clone() + self.next.clone();
        let current = mem::replace(&mut self.next, next);

        Some(mem::replace(&mut self.curr, current))
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
