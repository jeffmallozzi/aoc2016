use crate::{Solution, SolutionPair};
use itertools::{max, sorted};
use rfd::FileDialog;
use std::{collections::HashMap, fs::read_to_string, ops::Index};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let input_path = FileDialog::new().pick_file().expect("File Path Error");
    let input = read_to_string(input_path).expect("String Error");

    let sol: (u64, u64) = solution1(&input);

    (Solution::from(sol.0), Solution::from(sol.1))
}

fn checksum(char_count: HashMap<char, i32>) -> String {
    let mut char_vec: Vec<(char, i32)> = char_count.into_iter().collect();
    char_vec.sort_by(|a, b| sort_func(a.1, b.1, a.0, b.0));
    let mut sorted_chars: Vec<char> = Vec::new();
    for char_tup in char_vec {
        sorted_chars.push(char_tup.0);
    }

    sorted_chars[0..5].iter().collect()
}

fn sort_func(count_a: i32, count_b: i32, char_a: char, char_b: char) -> std::cmp::Ordering {
    if count_a > count_b {
        return std::cmp::Ordering::Less;
    }

    if count_a < count_b {
        return std::cmp::Ordering::Greater;
    }

    if char_a > char_b {
        return std::cmp::Ordering::Greater;
    }

    if char_a < char_b {
        return std::cmp::Ordering::Less;
    }

    std::cmp::Ordering::Equal
}

fn letter_shift(letter: char, mut shift: u64) -> char {
    if letter == ' ' {
        return ' ';
    }

    let letters: Vec<char> = ('a'..='z').collect();
    let codded_index = letters.iter().position(|&r| r == letter).unwrap();
    let decoded_index = (codded_index + shift as usize) % 26;

    letters[decoded_index]
}

fn decrypt(coded: &str, shift: u64) -> String {
    let mut decoded_chars: Vec<char> = Vec::new();

    for coded_char in coded.chars() {
        decoded_chars.push(letter_shift(coded_char, shift));
    }

    decoded_chars.iter().collect()
}

fn solution1(input: &str) -> (u64, u64) {
    let mut sector_id_sum: u64 = 0;
    let mut north_pole_storage_secotor_id: u64 = 0;
    for mut line in input.lines() {
        let mut line_vec: Vec<&str> = line.split_terminator(&['-', '[', ']']).collect();
        let given_checksum = line_vec.pop().expect("Empty").to_string();
        let sector_id: u64 = line_vec.pop().expect("Empty").parse().unwrap();

        let mut char_count = HashMap::new();

        for word in line_vec.iter() {
            for letter in word.chars() {
                *char_count.entry(letter).or_insert(0) += 1;
            }
        }

        let calc_checksum = checksum(char_count);

        if calc_checksum == given_checksum {
            sector_id_sum += sector_id;
            let codded_room = line_vec.join(" ");
            let decoded_room = decrypt(codded_room.as_str(), sector_id);
            if decoded_room.contains("north") {
                println!("Room: {}, Sector ID: {}", decoded_room, sector_id);
                north_pole_storage_secotor_id = sector_id;
            }
        }
    }
    (sector_id_sum, north_pole_storage_secotor_id)
}
