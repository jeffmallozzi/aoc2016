use crate::{Solution, SolutionPair};
use rfd::FileDialog;
use std::{collections::HashMap, env::current_exe, fs::read_to_string};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let input_path = FileDialog::new().pick_file().expect("File Path Error");
    let input = read_to_string(input_path).expect("String Error");

    let sol1: String = solution1(&input);
    let sol2: String = solution2(&input);

    (Solution::from(sol1), Solution::from(sol2))
}

fn next_button(current_button: u8, direction: char) -> u8 {
    match direction {
        'U' => move_up(current_button),
        'D' => move_down(current_button),
        'R' => move_right(current_button),
        'L' => move_left(current_button),
        _ => panic!("Invlalid direction"),
    }
}

fn move_up(current_button: u8) -> u8 {
    if [1, 2, 3].contains(&current_button) {
        return current_button;
    }

    current_button - 3
}

fn move_down(current_button: u8) -> u8 {
    if [7, 8, 9].contains(&current_button) {
        return current_button;
    }

    current_button + 3
}

fn move_right(current_button: u8) -> u8 {
    if [3, 6, 9].contains(&current_button) {
        return current_button;
    }

    current_button + 1
}

fn move_left(current_button: u8) -> u8 {
    if [1, 4, 7].contains(&current_button) {
        return current_button;
    }

    current_button - 1
}

fn solution1(input: &str) -> String {
    let mut current_button: u8 = 5;
    let lines = input.split(char::is_whitespace);
    let mut sequence = String::new();

    for mut line in lines {
        for next_move in line.chars() {
            current_button = next_button(current_button, next_move);
        }

        sequence.push_str(current_button.to_string().as_str());
    }
    sequence
}

#[derive(PartialEq, Eq, Hash, Clone)]
enum Button {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    A,
    B,
    C,
    D,
}

impl Button {
    fn to_string(&self) -> &str {
        match self {
            Button::One => "1",
            Button::Two => "2",
            Button::Three => "3",
            Button::Four => "4",
            Button::Five => "5",
            Button::Six => "6",
            Button::Seven => "7",
            Button::Eight => "8",
            Button::Nine => "9",
            Button::A => "A",
            Button::B => "B",
            Button::C => "C",
            Button::D => "D",
        }
    }
}

fn solution2(input: &str) -> String {
    use Button::*;
    let mut current_button: Button = Five;
    let lines = input.split(char::is_whitespace);
    let mut sequence = String::new();

    let move_maps = HashMap::from([
        (
            One,
            HashMap::from([('U', One), ('L', One), ('R', One), ('D', Three)]),
        ),
        (
            Two,
            HashMap::from([('U', Two), ('L', Two), ('R', Three), ('D', Six)]),
        ),
        (
            Three,
            HashMap::from([('U', One), ('L', Two), ('R', Four), ('D', Seven)]),
        ),
        (
            Four,
            HashMap::from([('U', Four), ('L', Three), ('R', Four), ('D', Eight)]),
        ),
        (
            Five,
            HashMap::from([('U', Five), ('L', Five), ('R', Six), ('D', Five)]),
        ),
        (
            Six,
            HashMap::from([('U', Two), ('L', Five), ('R', Seven), ('D', A)]),
        ),
        (
            Seven,
            HashMap::from([('U', Three), ('L', Six), ('R', Eight), ('D', B)]),
        ),
        (
            Eight,
            HashMap::from([('U', Four), ('L', Seven), ('R', Nine), ('D', C)]),
        ),
        (
            Nine,
            HashMap::from([('U', Nine), ('L', Eight), ('R', Nine), ('D', Nine)]),
        ),
        (A, HashMap::from([('U', Six), ('L', A), ('R', B), ('D', A)])),
        (
            B,
            HashMap::from([('U', Seven), ('L', A), ('R', C), ('D', D)]),
        ),
        (
            C,
            HashMap::from([('U', Eight), ('L', B), ('R', C), ('D', C)]),
        ),
        (D, HashMap::from([('U', B), ('L', D), ('R', D), ('D', D)])),
    ]);

    for mut line in lines {
        for next_move in line.chars() {
            let move_map = move_maps.get(&current_button).expect("Invalid Map");
            current_button = move_map.get(&next_move).expect("Invalid Move").clone();
        }

        sequence.push_str(current_button.to_string());
    }

    sequence.to_string()
}
