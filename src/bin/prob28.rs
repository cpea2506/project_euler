// Number Spiral Diagonals

trait Math {
    fn arithmetic_sum(self) -> usize;
    fn sum_of_square(self) -> usize;
    fn spiral_diagonals_sum(self) -> usize;
}

impl Math for usize {
    fn arithmetic_sum(self) -> usize {
        self * (self + 1) / 2
    }

    fn sum_of_square(self) -> usize {
        self * (self + 1) * (2 * self + 1) / 6
    }

    fn spiral_diagonals_sum(self) -> usize {
        4 * (4 * self.sum_of_square() + self.arithmetic_sum() + self) + 1
    }
}

fn spiral_diagonals_sum(length: usize) -> usize {
    assert!(length % 2 != 0, "The length of matrix must be odd!");

    let diagonal_len = length / 2;

    diagonal_len.spiral_diagonals_sum()
}

pj_euler::run!("Number Spiral Diagonals", spiral_diagonals_sum(1001));

pj_euler::test!(
    number_spiral_diagonals{
        {sum_spiral_diagonals_of_3x3_matrix, spiral_diagonals_sum(3), 25},
        {sum_spiral_diagonals_of_5x5_matrix, spiral_diagonals_sum(5), 101},
        {sum_spiral_diagonals_of_7x7_matrix, spiral_diagonals_sum(7), 261},
    }
);
