// Large Sum

fn ten_digits_of_large_sum() -> String {
    let data = include_str!("data.txt");

    data.lines()
        .filter_map(|l| l.parse::<f64>().ok())
        .sum::<f64>()
        .to_string()
        .chars()
        .take(10)
        .collect::<String>()
}

pj_euler::run!("Large Sum", ten_digits_of_large_sum());
