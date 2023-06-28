// Lattice Paths

use std::borrow::BorrowMut;

trait Math {
    fn doubled(self) -> Self;
}

impl Math for usize {
    fn doubled(self) -> Self {
        self * 2
    }
}

struct BinomialCoefficient {
    n: usize,
    k: usize,
}

impl BinomialCoefficient {
    fn new(n: usize, k: usize) -> Self {
        assert!(n > k, "n must be larger than k");

        Self { n, k }
    }

    /// Get the number of ways to choose an (unordered)
    /// subset of `k` elements from a fixed set of `n` elements
    fn value(self) -> usize {
        let mut n = self.n;

        (1..=self.k).borrow_mut().fold(1, |mut acc, k| {
            acc *= n;
            n -= 1;

            acc / k
        })
    }
}

fn lattice_path_counter(grid_size: usize) -> usize {
    BinomialCoefficient::new(grid_size.doubled(), grid_size).value()
}

pj_euler::run!("Lattice Paths", lattice_path_counter(20));

pj_euler::test!(
    lattice_paths {
        {grid_size_2_have_6_paths, lattice_path_counter(2), 6}
    }
);
