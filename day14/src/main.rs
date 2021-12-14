use std::fs;
use std::time::Instant;
use std::collections::HashMap;

type Replacements = HashMap<[u8; 2], [u8; 2]>;

fn main() {
    let start = Instant::now();
    println!("Part 1: {}", part1(&parse("input14.txt"))); // 4413
    println!("Part 2: {}", part2(&parse("input14.txt"))); // 118803
    let elapsed = start.elapsed();
    println!("Elapsed: {}µs", elapsed.as_micros());
}

fn parse(filename: &str) -> (String, Replacements) {
    let contents = fs::read_to_string(filename).expect("Error reading file!");
    let mut content_iter = contents.split("\n\n");
    let template = content_iter.next().unwrap().to_string();

    let mut replacements = HashMap::new();
    content_iter.next().unwrap().lines().for_each(|s| {
        let (pair, new) = s.split_once(" -> ").unwrap();
        let pair = pair.as_bytes();
        replacements.insert([pair[0], pair[1]], [pair[0], new.as_bytes()[0]]);
    });
    (template, replacements)
}

fn splice(polymer: &String, replacements: &Replacements) -> String {
    let mut pair: [u8; 2] = [0; 2];
    let bytes = polymer.as_bytes();
    let pair_count = polymer.len() - 1;
    let mut new_bytes = Vec::new();

    for i in 0..pair_count {
        pair[0] = bytes[i];
        pair[1] = bytes[i+1];
        let expanded = replacements.get(&pair).unwrap();
        new_bytes.push(expanded[0]);
        new_bytes.push(expanded[1]);
    }
    new_bytes.push(bytes[pair_count]);
    String::from_utf8(new_bytes).unwrap()
}

fn part1(system: &(String, Replacements)) -> usize {
    let (input, replacements) = system;
    let mut polymer = input.to_string();

    for i in 0..10 {
        println!("Polymer: {}", polymer);
        polymer = splice(&polymer, &replacements);
    }

    let mut counts = HashMap::new();
    for byte in polymer.bytes() {
        *counts.entry(byte).or_insert(0) += 1;
    }
    let min = counts.values().min().unwrap_or(&0);
    let max = counts.values().max().unwrap_or(&0);
    max - min
}

fn part2(system: &(String, Replacements)) -> usize { 
    0
}


#[test]
fn test_part1a() {
    assert_eq!(part1(&parse("sample.txt")), 1588);
}

#[test]
fn test_part2a() {
    assert_eq!(part2(&parse("sample.txt")), 16);
}
