// 1000-digit Fibonacci Number

struct Fibonacci {
    curr: usize,
    next: usize,
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci { curr: 0, next: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let new_value = self.curr + self.next;

        self.curr = self.next;
        self.next = new_value;

        Some(self.curr)
    }
}

fn thousand_digit_fibonacci_number(number_of_digit: usize) -> usize {
    let fibonacci = Fibonacci::new();

    fibonacci
        .filter(|&d| (d as f64).log10() as usize + 1 == number_of_digit)
        .count()
        + 1
}

pj_euler::run!(
    "1000-digit Fibonacci Number",
    thousand_digit_fibonacci_number(1000)
);

pj_euler::test!(
    thousand_digit_fibonacci_number {
        first_five_fibonacci {
            let fibo = Fibonacci::new();
            assert_eq!(fibo.take(5).collect::<Vec<usize>>(), &[1, 1, 2, 3, 5]);
        },
        first_three_digit_has_144 {
            assert_eq!(thousand_digit_fibonacci_number(3), 12);
        },
    }
);
