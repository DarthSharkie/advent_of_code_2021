use std::fs;
use std::str::FromStr;
use std::collections::VecDeque;
use std::collections::HashSet;

fn main() {
    println!("Part 1: {}", part1(&parse("input10.txt"))); // 518
    println!("Part 2: {}", part2(&parse("input10.txt"))); // 949905
}

fn parse(filename: &str) -> Floor {
    let contents = fs::read_to_string(filename).expect("Error reading file!");
    contents.trim().lines().map(|s| {

    }).collect()
}

fn part1() -> i32 {
    0
}

fn part2() -> i32 {
    0
}



#[test]
fn test_part1() {
    assert_eq!(part1(&parse("sample.txt")), 0);
}

#[test]
fn test_part2() {
    assert_eq!(part2(&parse("sample.txt")), 0);
}
