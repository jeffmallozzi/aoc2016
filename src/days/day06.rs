use crate::{Solution, SolutionPair};
use rfd::FileDialog;
use std::{array, collections::HashMap, fs::read_to_string};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let input_path = FileDialog::new().pick_file().expect("File Path Error");
    let input = read_to_string(input_path).expect("String Error");

    let sol: (String, String) = solution1(&input);

    (Solution::from(sol.0), Solution::from(sol.1))
}

fn solution1(input: &str) -> (String, String) {
    let mut letter_counts: [HashMap<char, u64>; 8] = core::array::from_fn(|i| HashMap::new());

    assert_eq!(letter_counts.len(), 8);

    for line in input.lines() {
        let mut char_index = 0;
        for (char_index, letter) in line.chars().enumerate() {
            *letter_counts[char_index].entry(letter).or_insert(0) += 1;
        }
    }

    let mut message: String = String::new();
    let mut message2: String = String::new();

    for letter_count in letter_counts {
        let mut letter_count_vec: Vec<(&char, &u64)> = letter_count.iter().collect();
        letter_count_vec.sort_by(|a, b| a.1.cmp(b.1));
        let mut most_frequent = letter_count_vec.pop().expect("Empty vec");
        message.push(*most_frequent.0);
        message2.push(*letter_count_vec[0].0);
    }

    (message, message2)
}
