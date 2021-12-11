use std::fs;
use std::collections::VecDeque;
use std::collections::HashSet;

fn main() {
    println!("Part 1: {}", part1(&parse("input11.txt"))); // 290691
    println!("Part 2: {}", part2(&parse("input11.txt"))); // 2768166558
}

fn parse(filename: &str) -> Nav {
    let contents = fs::read_to_string(filename).expect("Error reading file!");
    contents.trim().lines().map(|s| {
		s.chars().collect()
    }).collect()
}


#[test]
fn test_part1() {
    assert_eq!(part1(&parse("sample.txt")), 1656);
}

#[test]
fn test_part2() {
    assert_eq!(part2(&parse("sample.txt")), 288957);
}
