// Special Pythagorean triplet

fn pythagorean_triplet_product(triplet_sum: i32) -> Option<i32> {
    for c in triplet_sum / 3 + 1..triplet_sum / 2 {
        let square_difference_ab = c.pow(2) + 2 * triplet_sum * c - triplet_sum.pow(2);
        let difference_ab = (square_difference_ab as f64).sqrt();

        if difference_ab.trunc() == difference_ab {
            let b = (triplet_sum - c + difference_ab as i32) / 2;
            let a = triplet_sum - b - c;

            return Some(a * b * c);
        }
    }

    None
}

pj_euler::run!(
    "Special Pythagorean triplet",
    pythagorean_triplet_product(1000).unwrap()
);

pj_euler::test!(
    {triplet_3_4_5, pythagorean_triplet_product(12), Some(60)},
    {triplet_6_8_10, pythagorean_triplet_product(24), Some(480)},
    {triplet_7_24_25, pythagorean_triplet_product(56), Some(4200)},
    {triplet_not_find_out, pythagorean_triplet_product(3), None}
);
