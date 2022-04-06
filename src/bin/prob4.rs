// Largest palindrome product

#[allow(dead_code)]
fn is_palindrome(num: u32) -> bool {
    let mut temp = num;
    let mut sum = 0;

    while temp > 0 {
        sum = sum * 10 + (temp % 10);
        temp /= 10;
    }

    sum == num
}

fn largest_palidrome(digit_count: u32) -> u32 {
    let lower_bound = 10u32.pow(digit_count - 1);
    let upper_bound = lower_bound * 10 - 1;

    let mut a = upper_bound;
    let mut b: u32;
    let mut largest_palindrome = u32::MIN;

    while a >= lower_bound {
        b = upper_bound;

        while b >= a {
            let product = a * b;

            if product <= largest_palindrome {
                break;
            }

            if is_palindrome(product) {
                largest_palindrome = product
            }

            b -= 1
        }

        a -= 1;
    }

    largest_palindrome
}

pj_euler::run!("Largest palindrome product", largest_palidrome(3));

pj_euler::test!(
    {largest_palidrome_of_two_digits, largest_palidrome(2), 9009},
    {palindrome_test, is_palindrome(9009), true}
);
