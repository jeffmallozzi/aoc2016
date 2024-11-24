use std::{collections::HashMap, str::Bytes};

use crate::{Solution, SolutionPair};
use md5::compute;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let input: String = String::from("reyedfim");

    let sol1: String = solution1(&input);
    let sol2: String = solution2(&input);

    (Solution::from(sol1), Solution::from(sol2))
}

fn solution1(input: &str) -> String {
    let mut password: String = String::new();
    let mut int_index: u64 = 0;

    while password.len() < 8 {
        let mut test: String = String::from(input);
        test.push_str(&int_index.to_string());

        let test_hash = compute(test.as_bytes());
        let hash_hex = format!("{:02x}", test_hash).to_ascii_lowercase();

        if hash_hex.starts_with("00000") {
            password.push(hash_hex.chars().nth(5).expect("Not in the string"));
        }

        int_index += 1;
    }
    password
}

fn solution2(input: &str) -> String {
    let mut password_map: HashMap<char, char> = HashMap::new();
    let mut int_index: u64 = 0;

    while password_map.len() < 8 {
        let mut test: String = String::from(input);
        test.push_str(&int_index.to_string());

        let test_hash = compute(test.as_bytes());
        let hash_hex = format!("{:02x}", test_hash).to_ascii_lowercase();

        if hash_hex.starts_with("00000") {
            let pass_index = hash_hex.chars().nth(5).expect("Not a char");
            if pass_index.is_digit(8) {
                password_map
                    .entry(pass_index)
                    .or_insert(hash_hex.chars().nth(6).expect("Not a char"));
            }
        }

        int_index += 1;
    }
    let mut password_vec: Vec<(&char, &char)> = password_map.iter().collect();
    password_vec.sort_by_key(|k| k.0);

    let mut password: String = String::new();
    for pair in password_vec {
        password.push(*pair.1);
    }
    password
}
