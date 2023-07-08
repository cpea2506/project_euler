// Names Scores

use std::collections::HashMap;

type AlphabeticalOrder = HashMap<char, u8>;

struct Alphabet(AlphabeticalOrder);

impl Alphabet {
    fn new() -> Self {
        let mut alphabetical_order = AlphabeticalOrder::new();

        for c in 'A'..='Z' {
            alphabetical_order.insert(c, c as u8 - 64);
        }

        Self(alphabetical_order)
    }

    fn alphabetical_value(&self, name: &str) -> u8 {
        name.chars().fold(0, |acc, c| acc + self.0[&c])
    }
}

fn names_scores_sum() -> usize {
    let data = include_str!("data.txt").trim_end().replace('\"', "");
    let mut data = data.split(',').collect::<Vec<&str>>();
    data.sort_unstable();

    let alphabet = Alphabet::new();

    data.iter()
        .enumerate()
        .map(|(pos, name)| alphabet.alphabetical_value(name) as usize * (pos + 1))
        .sum()
}

pj_euler::run!("Names Scores", names_scores_sum());
pj_euler::test!(
    names_scores {
        colin_alphabetical_value {
            let alphabet = Alphabet::new();

            assert_eq!(alphabet.alphabetical_value("COLIN"), 53);
        },
        pamela_name_score_in_test_data {
            let data = include_str!("test.txt").trim_end().replace('\"', "");
            let mut data = data.split(',').collect::<Vec<&str>>();
            data.sort_unstable();

            let alphabet = Alphabet::new();
            let alphabetical_order = alphabet.alphabetical_value("PAMELA") as usize;
            let score =  alphabetical_order * (data.iter().position(|&name| name == "PAMELA").unwrap() + 1);

            assert_eq!(alphabetical_order, 48);
            assert_eq!(score, 240);
        }
});
