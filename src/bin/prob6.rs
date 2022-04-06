// Sum square difference

trait Math {
    fn sum_of_squares(&self) -> u32;
    fn square_of_sum(&self) -> u32;
}

impl Math for u32 {
    fn square_of_sum(&self) -> u32 {
        let sum_first_n = (self + 1) * self / 2;

        sum_first_n.pow(2)
    }

    fn sum_of_squares(&self) -> u32 {
        self * (self + 1) * (2 * self + 1) / 6
    }
}

fn sum_square_difference(n: u32) -> u32 {
    n.square_of_sum() - n.sum_of_squares()
}

pj_euler::run!("Sum square difference", sum_square_difference(100));

pj_euler::test!(
    {sum_of_squares_of_ten, 10.sum_of_squares(), 385},
    {square_of_sum_of_ten, 10.square_of_sum(), 3025},
    {first_ten, sum_square_difference(10), 2640}
);
