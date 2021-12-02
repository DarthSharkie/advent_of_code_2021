use std::fs;
use std::str::FromStr;
use std::num::ParseIntError;

struct Move {
    direction: String,
    distance: i32,
}

impl FromStr for Move {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data: Vec<&str> = s.trim().split(' ').collect();
        Ok(Move {direction: data[0].to_string(), distance: data[1].parse()?})
    }
}

fn main() {
    let filename = String::from("/mnt/s/AdventOfCode/2021/input02.txt");
    let contents = fs::read_to_string(filename).expect("Error reading file!");

    let moves: Vec<Move> = contents.trim().lines().map(|s| Move::from_str(s).unwrap()).collect();

    part1(&moves);
    part2(&moves);
}

fn part1(moves: &Vec<Move>) {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    for mv in moves {
        match mv.direction.as_str() {
            "forward" => x += mv.distance,
            "up" => y -= mv.distance,
            "down" => y += mv.distance,
            _ => println!("Bad input!"),
        }
    }
    println!("x: {}, y: {}, x*y: {}", x, y, x*y);
}

fn part2(moves: &Vec<Move>) {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut aim: i32 = 0;
    for mv in moves {
        match mv.direction.as_str() {
            "forward" => {
                x += mv.distance;
                y += (aim * mv.distance);
            },
            "up" => aim -= mv.distance,
            "down" => aim += mv.distance,
            _ => println!("Bad input!"),
        }
    }
    println!("x: {}, y: {}, x*y: {}", x, y, x*y);
}
