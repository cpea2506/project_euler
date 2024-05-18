// Number Spiral Diagonals

use number::sum::Sum;

fn spiral_diagonals_sum(length: usize) -> usize {
    assert!(length % 2 != 0, "The length of matrix must be odd!");

    let diagonal_len = length / 2;

    4 * (4 * diagonal_len.sum_of_squares() + diagonal_len.arithmetic_sum() + diagonal_len) + 1
}

pj_euler::run!("Number Spiral Diagonals", spiral_diagonals_sum(1001));

pj_euler::test!(
    number_spiral_diagonals{
        {sum_spiral_diagonals_of_3x3_matrix, spiral_diagonals_sum(3), 25},
        {sum_spiral_diagonals_of_5x5_matrix, spiral_diagonals_sum(5), 101},
        {sum_spiral_diagonals_of_7x7_matrix, spiral_diagonals_sum(7), 261},
    }
);
