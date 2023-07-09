// Lexicographic Permutations
// See: https://en.wikipedia.org/wiki/Factorial_number_system

type Number = usize;

fn factorial_numbers(limit: Number) -> Vec<Number> {
    let mut factorial_numbers = vec![1; limit];

    for n in 2..limit {
        factorial_numbers[n] = n * factorial_numbers[n - 1];
    }

    factorial_numbers
}

fn lexicographic_permutations_nth(list: &[Number], nth: usize) -> String {
    let mut list = list.to_vec();
    let factorial_numbers = factorial_numbers(list.len());
    let mut permutation = String::with_capacity(list.len());

    // zero-based
    let mut nth = nth - 1;

    while !list.is_empty() {
        let current_factorial = factorial_numbers[list.len() - 1];
        let pos = nth / current_factorial;

        permutation += &list[pos].to_string();
        list.remove(pos);

        nth %= current_factorial;
    }

    permutation
}

pj_euler::run!(
    "Lexicographic Permutations",
    lexicographic_permutations_nth(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9], 1_000_000)
);

pj_euler::test!(
    lexicographic_permutations {
        {third_lexicographic_permutation_of_012, lexicographic_permutations_nth(&[0, 1, 2], 3), "102"},
        {list_of_5_factorial_number_exclusive, factorial_numbers(5), vec![1, 1, 2, 6, 24]},
    }
);
