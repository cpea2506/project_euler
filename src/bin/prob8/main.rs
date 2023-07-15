// Largest product in a series

fn greatest_product(num_digits: usize) -> Option<usize> {
    let data = include_str!("data.txt");

    data.lines()
        .flat_map(|l| l.chars())
        .filter_map(|v| v.to_digit(10))
        .map(|v| v as usize)
        .collect::<Vec<usize>>()
        .windows(num_digits)
        .map(|w| w.iter().product())
        .max()
}

pj_euler::run!("Largest product in a series", greatest_product(13));

pj_euler::test!(
    largest_product_in_a_series {
        {greatest_product_of_4_adjacent_digits, greatest_product(4), Some(5832)}
    }
);
