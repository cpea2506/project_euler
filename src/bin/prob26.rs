// Reciprocal Cycles

fn longest_reciprocal_cycles(limit: u32) -> Option<u32> {
    (3..limit).max_by_key(|n| {
        if *n == 1 {
            return 1;
        }

        let mut decimal_frac = vec![None; *n as usize];
        let mut r = 1;
        let mut curr_index = 1u32;

        loop {
            r %= n;

            match decimal_frac[r as usize] {
                Some(i) => break curr_index - i,
                _ => decimal_frac[r as usize] = Some(curr_index),
            }

            r *= 10;
            curr_index += 1;
        }
    })
}

pj_euler::run!("Reciprocal Cycles", longest_reciprocal_cycles(1000));

pj_euler::test!(
    recprocal_cycles {
        {first_ten_numbers, longest_reciprocal_cycles(10), Some(7)},
    }
);
