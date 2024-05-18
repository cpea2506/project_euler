// Sum square difference

use number::sum::Sum;

fn sum_square_difference(n: u32) -> u32 {
    n.arithmetic_sum().pow(2) - n.sum_of_squares()
}

pj_euler::run!("Sum square difference", sum_square_difference(100));

pj_euler::test!(
    sum_square_difference {
        {first_ten, sum_square_difference(10), 2640},
    }
);
