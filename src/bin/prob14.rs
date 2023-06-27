// Longest Collatz Sequence

trait Math {
    fn is_even(&self) -> bool;
}

impl Math for usize {
    fn is_even(&self) -> bool {
        self % 2 == 0
    }
}

struct CollatzSequence {
    prev: Option<usize>,
    curr: usize,
}

#[allow(dead_code)]
impl CollatzSequence {
    fn new(starting_number: usize) -> Self {
        Self {
            prev: None,
            curr: starting_number,
        }
    }
}

impl Iterator for CollatzSequence {
    type Item = usize;

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

fn longest_collatz_sequence() -> usize {
    // (2..1_000_000)
    //     .max_by_key(|&v| CollatzSequence::new(v).count())
    //     .unwrap_or_default()

    // PERF: faster speed using caching approach
    let mut collatz_sequence = [0; 1_000_000];
    collatz_sequence[2] = 2;

    (3..collatz_sequence.len())
        .max_by_key(|&curr| {
            let mut prev = curr;
            let mut len = 0;

            while prev >= collatz_sequence.len() || collatz_sequence[prev] == 0 {
                if prev.is_even() {
                    prev /= 2;
                } else {
                    prev = 3 * prev + 1;
                }

                len += 1;
            }

            len += collatz_sequence[prev];

            collatz_sequence[curr] = len;

            len
        })
        .unwrap_or_default()
}

pj_euler::run!("Longest Collatz Sequence", longest_collatz_sequence());

pj_euler::test!(
    {collatz_sequence_start_at_13, CollatzSequence::new(13).collect::<Vec<usize>>(), vec![13, 40, 20, 10, 5, 16, 8, 4 ,2 , 1]}
);
