use crate::{Solution, SolutionPair};
use itertools::Itertools;
use rfd::FileDialog;
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let input_path = FileDialog::new().pick_file().expect("File Path Error");
    let input = read_to_string(input_path).expect("String Error");

    let sol1: u64 = solution1(&input);
    let sol2: u64 = solution2(&input);

    (Solution::from(sol1), Solution::from(sol2))
}

fn is_valid_triangle(a: u64, b: u64, c: u64) -> bool {
    let mut longest: u64 = 0;
    let mut other1: u64 = 0;
    let mut other2: u64 = 0;

    if a > b {
        longest = a;
        other1 = b;
    } else {
        longest = b;
        other1 = a;
    }

    if c > longest {
        other2 = longest;
        longest = c;
    } else {
        other2 = c;
    }

    if (other1 + other2) > longest {
        return true;
    }

    false
}

fn solution1(input: &str) -> u64 {
    let mut possible: u64 = 0;

    for mut line in input.lines() {
        let mut sides: Vec<&str> = line.split_whitespace().collect();
        let a: u64 = sides[0].parse().expect("Not a valid number");
        let b: u64 = sides[1].parse().expect("Not a valid number");
        let c: u64 = sides[2].parse().expect("Not a valid number");

        if is_valid_triangle(a, b, c) {
            possible += 1;
        }
    }
    possible
}

fn solution2(input: &str) -> u64 {
    let mut possible: u64 = 0;

    for three_lines in &input.lines().chunks(3) {
        let lines: Vec<_> = three_lines.collect();

        let mut triangle_1: Vec<u64> = Vec::new();
        let mut triangle_2: Vec<u64> = Vec::new();
        let mut triangle_3: Vec<u64> = Vec::new();

        for line in lines {
            let mut sides: Vec<&str> = line.split_whitespace().collect();
            triangle_1.push(sides[0].parse().expect("Not a valid number"));
            triangle_2.push(sides[1].parse().expect("Not a valid number"));
            triangle_3.push(sides[2].parse().expect("Not a valid number"));
        }

        if is_valid_triangle(triangle_1[0], triangle_1[1], triangle_1[2]) {
            possible += 1
        }
        if is_valid_triangle(triangle_2[0], triangle_2[1], triangle_2[2]) {
            possible += 1
        }
        if is_valid_triangle(triangle_3[0], triangle_3[1], triangle_3[2]) {
            possible += 1
        }
    }

    possible
}
