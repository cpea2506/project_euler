// Even Fibonacci numbers

use sequence::fibonacci::Fibonacci;

fn sum_not_exceed(bound: i32) -> i32 {
    Fibonacci::new()
        .take_while(|&f| f < bound)
        .filter(|f| f % 2 == 0)
        .sum()
}

pj_euler::run!("Even Fibonacci numbers", sum_not_exceed(4_000_000));

pj_euler::test!(
   even_fibonacci_numbers {
       {sum_below_five, sum_not_exceed(5), 2}, // 2
       {sum_below_ten, sum_not_exceed(10), 10} // 2 and 8
   }
);
