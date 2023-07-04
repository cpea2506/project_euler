// Maximum Path Sum I

type Vec2D<T> = Vec<Vec<T>>;

fn maximum_path_sum(with_test_example: bool) -> usize {
    let data = if !with_test_example {
        include_str!("data.txt")
    } else {
        include_str!("test.txt")
    };

    let mut vec_data = data
        .lines()
        .map(|l| {
            l.split_whitespace()
                .filter_map(|v| v.parse().ok())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec2D<usize>>();

    for row in (1..vec_data.len()).rev() {
        for col in 0..vec_data[row].len() - 1 {
            let left = vec_data[row][col];
            let right = vec_data[row][col + 1];

            vec_data[row - 1][col] += left.max(right);
        }
    }

    vec_data[0][0]
}

pj_euler::run!("Maximum Path Sum I", maximum_path_sum(false));

pj_euler::test!(
    maximum_path_sum_i {
        {test_example, maximum_path_sum(true), 23}
    }
);
