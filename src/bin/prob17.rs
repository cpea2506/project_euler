// Number Letter Counts

use lazy_static::lazy_static;
use std::{collections::HashMap, ops::RangeInclusive};

lazy_static! {
    static ref NUMBER_LETTER_COUNT: HashMap<usize, usize> = HashMap::from([
        (0, 0), // ignore zero
        (1, "one".len()),
        (2, "two".len()),
        (3, "three".len()),
        (4, "four".len()),
        (5, "five".len()),
        (6, "six".len()),
        (7, "seven".len()),
        (8, "eight".len()),
        (9, "nine".len()),
        (10, "ten".len()),
        (11, "eleven".len()),
        (12, "twelve".len()),
        (13, "thirteen".len()),
        (14, "fourteen".len()),
        (15, "fifteen".len()),
        (16, "sixteen".len()),
        (17, "seventeen".len()),
        (18, "eighteen".len()),
        (19, "nineteen".len()),
        (20, "twenty".len()),
        (30, "thirty".len()),
        (40, "forty".len()),
        (50, "fifty".len()),
        (60, "sixty".len()),
        (70, "seventy".len()),
        (80, "eighty".len()),
        (90, "ninety".len()),
        (100, "hundred".len()),
        (1000, "thousand".len()),
    ]);
}

trait Letter {
    const WORD_AND_LEN: usize = 3;

    fn letter_count(self) -> usize;
}

impl Letter for usize {
    fn letter_count(self) -> usize {
        let mut number = self;
        let mut count = 0;

        let thousand = number / 1000;
        if thousand >= 1 {
            count += NUMBER_LETTER_COUNT[&1000] + NUMBER_LETTER_COUNT[&thousand];
            number %= 1000;
        }

        let hundred = number / 100;
        if hundred >= 1 {
            count += NUMBER_LETTER_COUNT[&100] + NUMBER_LETTER_COUNT[&hundred];
            number %= 100;

            if number >= 1 {
                count += Self::WORD_AND_LEN
            }
        }

        if number < 20 {
            count + NUMBER_LETTER_COUNT[&number]
        } else {
            count + NUMBER_LETTER_COUNT[&(number / 10 * 10)] + NUMBER_LETTER_COUNT[&(number % 10)]
        }
    }
}

fn number_letter_counts(range: RangeInclusive<usize>) -> usize {
    range.fold(0, |acc, num| acc + num.letter_count())
}

pj_euler::run!("Number Letter Counts", number_letter_counts(1..=1000));

pj_euler::test!(
    number_letter_counts {
        {count_first_5, number_letter_counts(1..=5), 19},
        {count_only_342, 342.letter_count(), 23},
        {count_only_202, 202.letter_count(), 16},
        {count_only_400, 400.letter_count(), 11},
        {count_only_115, 115.letter_count(), 20},
        {count_only_1000, 1000.letter_count(), 11},
    }
);
