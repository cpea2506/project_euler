use std::ops::AddAssign;

use num_traits::{One, Zero};

pub struct TriangularSequence<T> {
    sum: T,
    curr: T,
}

impl<T> Default for TriangularSequence<T>
where
    T: One + Zero,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> TriangularSequence<T>
where
    T: One + Zero,
{
    pub fn new() -> Self {
        Self {
            sum: Zero::zero(),
            curr: One::one(),
        }
    }
}

impl<T> Iterator for TriangularSequence<T>
where
    T: AddAssign + One + Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.sum += self.curr;
        self.curr += One::one();

        Some(self.sum)
    }
}
