use num_integer::Integer;
use num_traits::{One, Zero};

pub struct Collatz<T> {
    prev: Option<T>,
    curr: T,
}

impl<T> Default for Collatz<T>
where
    T: One + Zero,
{
    fn default() -> Self {
        Self::new(One::one())
    }
}

impl<T> Collatz<T> {
    #[inline]
    pub fn new(starting_number: T) -> Self {
        Self {
            prev: None,
            curr: starting_number,
        }
    }
}

impl<T: Integer + Copy> Iterator for Collatz<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.curr == One::one() {
            None
        } else {
            let one: T = One::one();
            let two = one + one;
            let three = one + two;

            if let Some(prev) = self.prev {
                self.curr = if prev.is_even() {
                    prev / two
                } else {
                    prev * three + one
                };
            }

            self.prev = Some(self.curr);

            self.prev
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collatz() {
        assert_eq!(
            Collatz::new(13).collect::<Vec<u32>>(),
            &[13, 40, 20, 10, 5, 16, 8, 4, 2, 1],
        );
    }
}
