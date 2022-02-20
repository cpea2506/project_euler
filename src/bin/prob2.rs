// Even Fibonacci numbers

struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci { curr: 1, next: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let new_value = self.curr + self.next;

        self.curr = self.next;
        self.next = new_value;

        Some(self.curr)
    }
}

fn sum_not_exceed(bound: u32) -> u32 {
    let fibo = Fibonacci::new();

    fibo.take_while(|&f| f < bound).filter(|f| f % 2 == 0).sum()
}

pj_euler::solution!("Even Fibonacci numbers", sum_not_exceed(4_000_000));

pj_euler::test!(
   {sum_below_five, sum_not_exceed(5), 2}, // 2
   {sum_below_ten, sum_not_exceed(10), 10} // 2 and 8
);
