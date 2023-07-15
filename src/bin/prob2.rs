// Even Fibonacci numbers

use num_integer::Integer;
use sequence::fibonacci::Fibonacci;

fn sum_not_exceed(bound: u32) -> u32 {
    Fibonacci::<u32>::new()
        .take_while(|&f| f < bound)
        .filter(Integer::is_even)
        .sum()
}

pj_euler::run!("Even Fibonacci numbers", sum_not_exceed(4_000_000));

pj_euler::test!(
   even_fibonacci_numbers {
       {sum_below_five, sum_not_exceed(5), 2},
       {sum_below_ten, sum_not_exceed(10), 10},
   }
);
