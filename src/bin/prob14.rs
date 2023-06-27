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
    let (mut longest_chain, mut starting_number) = (usize::MIN, 2);

    (starting_number..1_000_000).for_each(|v| {
        let collatz_sequence = CollatzSequence::new(v);
        let count = collatz_sequence.count();

        if count > longest_chain {
            longest_chain = count;
            starting_number = v;
        }
    });

    starting_number
}

pj_euler::run!("Longest Collatz Sequence", longest_collatz_sequence());

pj_euler::test!(
    {collatz_sequence_start_at_13, CollatzSequence::new(13).collect::<Vec<usize>>(), vec![13, 40, 20, 10, 5, 16, 8, 4 ,2 , 1]}
);
