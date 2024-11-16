use crate::{Solution, SolutionPair};
use rfd::FileDialog;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::hash::Hash;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let input_path = FileDialog::new().pick_file().expect("File Path Error");
    let input = read_to_string(input_path).expect("String Error");

    let sol1: i32 = part1(&input);
    let sol2: i32 = part2(&input);

    (Solution::from(sol1), Solution::from(sol2))
}

enum Directions {
    North,
    East,
    South,
    West,
}

impl Directions {
    fn turnleft(&mut self) -> Directions {
        match &self {
            Directions::North => Directions::West,
            Directions::East => Directions::North,
            Directions::South => Directions::East,
            Directions::West => Directions::South,
        }
    }

    fn turnright(&mut self) -> Directions {
        match &self {
            Directions::North => Directions::East,
            Directions::East => Directions::South,
            Directions::South => Directions::West,
            Directions::West => Directions::North,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Location {
    x: i32,
    y: i32,
}

impl Location {
    fn move_along(&mut self, direction: &Directions, distance: i32) -> &Location {
        match direction {
            Directions::North => self.y += distance,
            Directions::East => self.x += distance,
            Directions::South => self.y -= distance,
            Directions::West => self.x -= distance,
        }
        self
    }
}

fn part1(input: &str) -> i32 {
    let steps = input.split(',');
    let mut northsouth: i32 = 0;
    let mut eastwest: i32 = 0;
    // direction: 0 North, 1 East, 2 South, 3 West
    let mut direction: Directions = Directions::North;

    for mut step in steps {
        step = step.trim();
        if step.starts_with("L") {
            direction = direction.turnleft();
        } else {
            direction = direction.turnright();
        }

        let distance: i32 = step
            .trim_start_matches(char::is_alphabetic)
            .parse()
            .unwrap();

        match direction {
            Directions::North => northsouth += distance,
            Directions::East => eastwest += distance,
            Directions::South => northsouth -= distance,
            Directions::West => eastwest -= distance,
        }
    }

    northsouth.abs() + eastwest.abs()
}

fn part2(input: &str) -> i32 {
    let steps = input.split(',');
    let mut current_location = Location { x: 0, y: 0 };
    let mut direction: Directions = Directions::North;
    let mut locations = HashSet::new();
    locations.insert(current_location.clone());

    for mut step in steps {
        step = step.trim();
        if step.starts_with("L") {
            direction = direction.turnleft();
        } else {
            direction = direction.turnright();
        }

        let distance: i32 = step
            .trim_start_matches(char::is_alphabetic)
            .parse()
            .unwrap();

        for _ in 0..distance {
            current_location.move_along(&direction, 1);
            if locations.contains(&current_location) {
                return current_location.x.abs() + current_location.y.abs();
            } else {
                locations.insert(current_location.clone());
            }
        }
    }

    current_location.x.abs() + current_location.y.abs()
}
