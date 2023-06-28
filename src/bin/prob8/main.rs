// Largest product in a series

fn greatest_product(num_digits: usize) -> usize {
    let data = include_str!("data.txt");

    let vec_data = data
        .lines()
        .flat_map(|l| l.chars())
        .map(|v| v.to_digit(10).unwrap() as usize)
        .collect::<Vec<usize>>();

    vec_data
        .windows(num_digits)
        .map(|w| w.iter().product())
        .max()
        .unwrap_or_default()
}

pj_euler::run!("Largest product in a series", greatest_product(13));

pj_euler::test!(
    largest_product_in_a_series {
        {greatest_product_of_4_adjacent_digits, greatest_product(4), 5832}
    }
);
