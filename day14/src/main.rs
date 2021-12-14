use std::fs;
use std::time::Instant;
use std::collections::HashMap;

type Replacements = HashMap<[u8; 2], u8>;
type Counts = HashMap<u8, usize>;
type Memos = HashMap<([u8; 2], usize), Counts>; 

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
        replacements.insert([pair[0], pair[1]], new.as_bytes()[0]);
    });
    (template, replacements)
}

fn splice(pair: [u8; 2], replacements: &Replacements, memos: &mut Memos, iterations: usize) -> Counts {

    // if we've see the pair and iteration count before, short-cut
    if let Some(memo) = memos.get(&(pair, iterations)) {
        memo.clone()
    } else {
        let new_byte = replacements.get(&pair).unwrap();
        let first_pair = [pair[0], *new_byte];
        let second_pair = [*new_byte, pair[1]];

        let mut counts = Counts::new();
        // Insert counts

        
        if iterations == 1 {
            *counts.entry(pair[0]).or_insert(0) += 1;
            *counts.entry(*new_byte).or_insert(0) += 1;
            *counts.entry(pair[1]).or_insert(0) += 1;
        } else {
            // pair[0] + new_byte
            let count1 = splice(first_pair, &replacements, memos, iterations - 1);
            count1.iter().for_each(|(&byte, count)| *counts.entry(byte).or_insert(0) += count);

            // New byte will be double-counted
            *counts.entry(*new_byte).or_insert(0) -= 1;

            // expanded + pair[1]
            let count2 = splice(second_pair, &replacements, memos, iterations - 1);
            count2.iter().for_each(|(&byte, count)| *counts.entry(byte).or_insert(0) += count);
        }
        // Create memo
        memos.insert((pair, iterations), counts.clone());
        counts
    }
}

fn part1(system: &(String, Replacements)) -> usize {
    let (polymer, replacements) = system;
    expand(polymer, replacements, 10)
}

fn expand(polymer: &String, replacements: &Replacements, iterations: usize) -> usize {
    let mut counts = Counts::new();
    let mut memos = Memos::new();

    let bytes = polymer.as_bytes();
    for i in 0..polymer.len() - 1 {
        let pair_counts = splice([bytes[i], bytes[i+1]], &replacements, &mut memos, iterations);
        pair_counts.iter().for_each(|(&byte, count)| *counts.entry(byte).or_insert(0) += count);
    }

    println!("Counts: {:?}", counts);

    let min = counts.values().min().unwrap_or(&0);
    let max = counts.values().max().unwrap_or(&0);
    max - min
}

fn part2(system: &(String, Replacements)) -> usize { 
    let (polymer, replacements) = system;
    expand(polymer, replacements, 40)
}


#[test]
fn test_part1a() {
    assert_eq!(part1(&parse("sample.txt")), 1588);
}

#[test]
fn test_part2a() {
    assert_eq!(part2(&parse("sample.txt")), 2188189693529);
}
