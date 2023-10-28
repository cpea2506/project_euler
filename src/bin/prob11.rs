// Largest Product in a Grid

const SIZE: usize = 20;
const DX: [isize; 4] = [1, 0, 1, -1];
const DY: [isize; 4] = [0, 1, 1, 1];

fn largest_product_in_grid(num_digits: usize) -> usize {
    let data = include_str!("../data/prob11.txt");

    let vec_data = data
        .lines()
        .flat_map(|l| l.split_whitespace().flat_map(|v| v.parse()))
        .collect::<Vec<usize>>();

    let mut max_product = usize::MIN;

    (0..SIZE).for_each(|r| {
        (0..SIZE).for_each(|c| {
            let product = (0..4)
                .map(|d| {
                    (0..num_digits).fold(1, |acc, v| {
                        let row = (r as isize + v as isize * DY[d]) as usize;
                        let col = (c as isize + v as isize * DX[d]) as usize;

                        if row < SIZE && col < SIZE {
                            acc * vec_data[col + row * SIZE]
                        } else {
                            0
                        }
                    })
                })
                .max()
                .unwrap_or_default();

            max_product = max_product.max(product);
        })
    });

    max_product
}

pj_euler::run!("Largest Product in a Grid", largest_product_in_grid(4));
