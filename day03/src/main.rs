use std::fs;
use std::str::FromStr;

// Replace with day-specific struct/enums
struct Move {
    direction: String,
    distance: i32,
}

impl FromStr for Move {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data: Vec<&str> = s.trim().split(' ').collect();
        // Add day-specific struct building

    }
}

fn main() {
    let filename = String::from("input03.txt");
    let contents = fs::read_to_string(filename).expect("Error reading file!");

    let moves: Vec<Move> = contents.trim().lines().map(|s| s.parse().unwrap()).collect();

    part1(&moves);
    part2(&moves);
}

fn part1(moves: &[Move]) {
}

fn part2(moves: &[Move]) {
}
