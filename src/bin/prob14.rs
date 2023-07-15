// Longest Collatz Sequence

use sequence::collatz::Collatz;
use std::ops::Range;

fn longest_collatz_sequence(range: Range<usize>) -> Option<usize> {
    let end = range.end;

    let mut collatz_sequence_map = vec![0; end];
    collatz_sequence_map[2] = 2;

    range.max_by_key(|&value| {
        let collatz_iter = Collatz::new(value);
        let mut prev = 0;

        let mut len = collatz_iter
            .take_while(|&v| {
                prev = v;

                v >= end || collatz_sequence_map[v] == 0
            })
            .count();

        len += collatz_sequence_map[prev];

        collatz_sequence_map[value] = len;

        len
    })
}

pj_euler::run!(
    "Longest Collatz Sequence",
    longest_collatz_sequence(2..1_000_000)
);
