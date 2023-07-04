// Longest Collatz Sequence

use std::ops::Range;

trait Math {
    fn is_even(&self) -> bool;
}

impl Math for usize {
    fn is_even(&self) -> bool {
        self % 2 == 0
    }
}

type Number = usize;

struct CollatzSequence {
    prev: Option<Number>,
    curr: Number,
}

impl CollatzSequence {
    fn new(starting_number: Number) -> Self {
        Self {
            prev: None,
            curr: starting_number,
        }
    }
}

impl Iterator for CollatzSequence {
    type Item = Number;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr == 1 {
            None
        } else {
            if let Some(prev) = self.prev {
                self.curr = if prev.is_even() {
                    prev / 2
                } else {
                    prev * 3 + 1
                };
            }

            self.prev = Some(self.curr);

            self.prev
        }
    }
}

fn longest_collatz_sequence(range: Range<Number>) -> Number {
    let end = range.end;

    let mut collatz_sequence_map = vec![0; end];
    collatz_sequence_map[2] = 2;

    range
        .max_by_key(|&value| {
            let collatz_iter = CollatzSequence::new(value);
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
        .unwrap_or_default()
}

pj_euler::run!(
    "Longest Collatz Sequence",
    longest_collatz_sequence(2..1_000_000)
);

pj_euler::test!(
    longest_collatz_sequence {
        {collatz_sequence_start_at_13, CollatzSequence::new(13).collect::<Vec<usize>>(), vec![13, 40, 20, 10, 5, 16, 8, 4, 2, 1]}
    }
);
