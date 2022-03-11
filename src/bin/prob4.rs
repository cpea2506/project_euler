// Largest palindrome product

fn is_palindrome(num: u32) -> bool {
    let mut temp = num;
    let mut sum = 0;

    while temp > 0 {
        sum = sum * 10 + (temp % 10);
        temp /= 10;
    }

    sum == num
}

// abba = 11(91a + 10b)
fn largest_palidrome(digit_count: u32) -> u32 {
    let lower_bound = 10u32.pow(digit_count - 1);
    let _upper_bound = lower_bound * 10 - 1;

    0
}

pj_euler::solution!("Largest palindrome product", largest_palidrome(3));

pj_euler::test!(
    {largest_palidrome_of_two_digits, largest_palidrome(20), 9009},
    {palindrome_test, is_palindrome(9009), true}
);
